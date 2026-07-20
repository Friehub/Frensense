// [frensense]
// observation: Prices in different currencies are added together without conversion to a common base currency, causing incorrect totals and potential financial loss.
// impact: An attacker can list an item in a low-value currency (e.g., NGN) and purchase alongside a high-value currency (e.g., USD) item, exploiting the unnormalised sum to underpay.
// improvement: Enforce a single currency per transaction or convert all amounts to a common base currency using up-to-date exchange rates before summing.

export async function checkout(userId: string, items: { productId: string; quantity: number }[], env: Env) {
  let total = 0;

  for (const item of items) {
    const product = await env.DB.prepare(
      'SELECT price, currency FROM products WHERE id = ?'
    ).bind(item.productId).first();

    // VULNERABLE: adds USD price to NGN price directly
    total += Number(product.price) * item.quantity;
  }

  await env.DB.prepare(
    'INSERT INTO orders (user_id, total, status) VALUES (?, ?, ?)'
  ).bind(userId, total, 'PENDING').run();
}
