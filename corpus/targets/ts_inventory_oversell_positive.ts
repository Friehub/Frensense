// [frensense]
// observation: The stock availability check and the stock decrement are separate database operations, creating a TOCTOU race condition that allows overselling.
// impact: Concurrent purchase requests can both see available stock and both decrement it, resulting in more items sold than available and forcing the merchant to cancel orders.
// improvement: Use a single atomic UPDATE with a stock guard (WHERE stock >= qty) or use a database transaction with row-level locking.
// cwe: CWE-841
// cvss: 6.5
// owasp: 
// severity: Medium

export async function purchaseProduct(productId: string, quantity: number, userId: string, env: Env) {
  // VULNERABLE: read stock, then decrement — race window
  const product = await env.DB.prepare(
    'SELECT stock FROM products WHERE id = ?'
  ).bind(productId).first();

  if (!product || product.stock < quantity) {
    throw new Error('Insufficient stock');
  }

  await env.DB.prepare(
    'UPDATE products SET stock = stock - ? WHERE id = ?'
  ).bind(quantity, productId).run();

  await env.DB.prepare(
    'INSERT INTO orders (user_id, product_id, quantity, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, productId, quantity, 'PENDING').run();
}
