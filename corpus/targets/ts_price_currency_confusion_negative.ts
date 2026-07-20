// SAFE: Converts all item prices to a common currency using exchange rates before summing

export async function checkout(userId: string, items: { productId: string; quantity: number }[], env: Env) {
  let total = 0;
  const baseCurrency = 'USD';

  for (const item of items) {
    const product = await env.DB.prepare(
      'SELECT price, currency FROM products WHERE id = ?'
    ).bind(item.productId).first();

    let price = Number(product.price);

    // SAFE: convert to base currency if needed
    if (product.currency !== baseCurrency) {
      const rate = await getExchangeRate(product.currency, baseCurrency, env);
      price *= rate;
    }

    total += price * item.quantity;
  }

  await env.DB.prepare(
    'INSERT INTO orders (user_id, total, currency, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, total, baseCurrency, 'PENDING').run();
}

async function getExchangeRate(from: string, to: string, env: Env): Promise<number> {
  const res = await fetch(
    `https://api.exchangerate.host/convert?from=${from}&to=${to}`
  );
  const data = await res.json();
  return data.result;
}
