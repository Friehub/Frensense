// SAFE: publish event after state change
import { eventBus } from './event-bus';

async function confirmOrder(orderId: string, db: DB): Promise<void> {
  await db.query('UPDATE orders SET status = $1 WHERE id = $2', ['confirmed', orderId]);
  await eventBus.publish('order.confirmed', { orderId, confirmedAt: new Date() });
}

async function activateSubscription(subscriptionId: string, db: DB): Promise<void> {
  await db.query('UPDATE subscriptions SET status = $1 WHERE id = $2', ['active', subscriptionId]);
  await eventBus.publish('subscription.activated', { subscriptionId, activatedAt: new Date() });
}

async function processRefund(refundId: string, db: DB): Promise<void> {
  await db.query('UPDATE refunds SET status = $1 WHERE id = $2', ['completed', refundId]);
  await eventBus.publish('refund.completed', { refundId, completedAt: new Date() });
}
