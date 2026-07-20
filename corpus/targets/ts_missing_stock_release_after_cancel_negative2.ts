// SAFE alternative: domain event triggers inventory release
async function cancelOrder(orderId: string, db: DB): Promise<void> {
  await db.query('UPDATE orders SET status = $1 WHERE id = $2', ['cancelled', orderId]);
  await eventBus.publish('order.cancelled', { orderId });
}

eventBus.on('order.cancelled', async (event) => {
  const items = await db.query('SELECT product_id, quantity FROM order_items WHERE order_id = $1', [event.orderId]);
  for (const item of items) {
    await db.query(
      'UPDATE inventory SET reserved = GREATEST(reserved - $1, 0), available = available + $1 WHERE product_id = $2',
      [item.quantity, item.product_id]
    );
  }
});
