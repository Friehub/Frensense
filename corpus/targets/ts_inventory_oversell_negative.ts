// SAFE: Uses a single atomic UPDATE with a stock guard to prevent overselling

export async function purchaseProduct(productId: string, quantity: number, userId: string, env: Env) {
  // SAFE: atomic decrement with stock guard
  const result = await env.DB.prepare(
    'UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?'
  ).bind(quantity, productId, quantity).run();

  if (result.meta.changes === 0) {
    throw new Error('Insufficient stock');
  }

  await env.DB.prepare(
    'INSERT INTO orders (user_id, product_id, quantity, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, productId, quantity, 'PENDING').run();
}
