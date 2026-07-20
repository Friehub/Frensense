// SAFE alternative: transactional outbox pattern
import { db } from './db';
import { outbox } from './outbox';

async function confirmOrder(orderId: string): Promise<void> {
  await db.transaction(async (tx) => {
    await tx.query('UPDATE orders SET status = $1 WHERE id = $2', ['confirmed', orderId]);
    await outbox.add(tx, 'order.confirmed', { orderId });
  });
}
