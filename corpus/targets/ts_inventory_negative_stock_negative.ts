// SAFE: Uses a WHERE clause to prevent stock from going below zero

export async function deductStock(productId: string, quantity: number, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?'
  ).bind(quantity, productId, quantity).run();

  if (result.meta.changes === 0) {
    throw new Error('Insufficient stock');
  }
}
