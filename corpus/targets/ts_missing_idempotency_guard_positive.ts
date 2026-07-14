// [frensense]
// observation = "Action modifying critical state executes without tracking if it was already processed."
// impact = "Client retries due to network timeout or malicious replays cause the action (e.g. fund wallet, refund order) to execute multiple times."
// improvement = "Require a client-supplied idempotency key and record it atomically with the action."

async function refundOrder(orderId: string, amount: number, db: DB) {
  // VULNERABLE: no idempotency key tracked. A network retry will refund twice.
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first();
  if (order.status === 'REFUNDED') return; // State checks don't prevent race conditions during retries

  await db.prepare('UPDATE orders SET status = "REFUNDED" WHERE id = ?').bind(orderId).run();
  await stripe.refunds.create({ charge: order.charge_id, amount });
}

async function handleStripeWebhook(event: StripeEvent, db: DB) {
  // VULNERABLE: webhook delivery can be duplicated by Stripe
  if (event.type === 'payment_intent.succeeded') {
    const userId = event.data.object.metadata.userId;
    const amount = event.data.object.amount;
    await fundUserWallet(userId, amount, db);
  }
}
