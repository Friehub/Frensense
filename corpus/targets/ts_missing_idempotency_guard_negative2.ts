// SAFE: Uses Redis SET NX for atomic idempotency key deduplication
import redis from "./redis";

async function refundOrder(orderId: string, amount: number, idempotencyKey: string, db: DB) {
  const key = `idempotent:refund:${idempotencyKey}`;
  const stored = await redis.set(key, "1", { NX: true, EX: 86400 });
  if (!stored) return { status: "already_processed" };
  await db.prepare('UPDATE orders SET status = "REFUNDED" WHERE id = ?').bind(orderId).run();
  await stripe.refunds.create({ charge: orderId, amount }, { idempotencyKey });
}

async function handleStripeWebhook(event: StripeEvent, db: DB) {
  const key = `webhook:${event.id}`;
  const stored = await redis.set(key, "1", { NX: true, EX: 86400 });
  if (!stored) return new Response("Already processed", { status: 200 });
  if (event.type === "payment_intent.succeeded") {
    const userId = event.data.object.metadata.userId;
    const amount = event.data.object.amount;
    await fundUserWallet(userId, amount, db);
  }
}
