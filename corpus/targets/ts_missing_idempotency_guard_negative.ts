// SAFE: idempotency key is required and checked before side effects
async function refundOrder(orderId: string, amount: number, idempotencyKey: string, db: DB) {
  // SAFE: atomic insertion of idempotency key prevents duplicate processing
  try {
    await db.prepare('INSERT INTO idempotency_keys (key, action) VALUES (?, ?)')
      .bind(idempotencyKey, `refund_${orderId}`).run();
  } catch (e) {
    if (e.message.includes('UNIQUE constraint failed')) {
      return { status: 'already_processed' };
    }
    throw e;
  }

  await db.prepare('UPDATE orders SET status = "REFUNDED" WHERE id = ?').bind(orderId).run();
  await stripe.refunds.create({ charge: order.charge_id, amount }, { idempotencyKey });
}

async function handleStripeWebhook(event: StripeEvent, db: DB) {
  // SAFE: webhook event IDs are globally unique and can be used for idempotency
  try {
    await db.prepare('INSERT INTO processed_webhooks (id) VALUES (?)').bind(event.id).run();
  } catch (e) {
    return new Response('Already processed', { status: 200 });
  }

  if (event.type === 'payment_intent.succeeded') {
    const userId = event.data.object.metadata.userId;
    const amount = event.data.object.amount;
    await fundUserWallet(userId, amount, db);
  }
}
