// SAFE: Checks for overflow before performing the multiplication, using BigInt for safety

export async function checkoutCart(userId: string, items: { productId: string; quantity: number }[], env: Env) {
  let total = BigInt(0);

  for (const item of items) {
    const product = await env.DB.prepare(
      'SELECT price FROM products WHERE id = ?'
    ).bind(item.productId).first();

    const priceBig = BigInt(Math.round(Number(product.price) * 100));
    const qtyBig = BigInt(item.quantity);
    const lineTotal = priceBig * qtyBig;

    // SAFE: check for overflow beyond DB column capacity (BIGINT)
    if (lineTotal > BigInt('9223372036854775807')) {
      throw new Error(`Line total exceeds maximum for ${item.productId}`);
    }

    total += lineTotal;
  }

  await env.DB.prepare(
    'INSERT INTO orders (user_id, total_cents, status) VALUES (?, ?, ?)'
  ).bind(userId, Number(total), 'PENDING').run();
}
