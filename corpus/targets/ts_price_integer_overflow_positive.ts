// [frensense]
// observation: The price multiplied by quantity uses a standard JavaScript number type without overflow protection, allowing the total to silently overflow past Number.MAX_SAFE_INTEGER.
// impact: An attacker can order a massive quantity of a low-priced item to cause an integer overflow, resulting in a very small total and paying far less than expected.
// improvement: Use BigInt for multiplication, validate that quantity * price does not exceed Number.MAX_SAFE_INTEGER, or use PostgreSQL NUMERIC/BigInt columns.
// cwe: CWE-190
// cvss: 7.5
// owasp: 
// severity: High

export async function checkoutCart(userId: string, items: { productId: string; quantity: number }[], env: Env) {
  let total = 0;

  for (const item of items) {
    const product = await env.DB.prepare(
      'SELECT price FROM products WHERE id = ?'
    ).bind(item.productId).first();

    // VULNERABLE: no overflow check; 1e10 * 1e6 overflows Number
    total += Number(product.price) * item.quantity;
  }

  if (total <= 0) throw new Error('Invalid total');

  await env.DB.prepare(
    'INSERT INTO orders (user_id, total, status) VALUES (?, ?, ?)'
  ).bind(userId, total, 'PENDING').run();
}
