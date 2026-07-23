// SAFE: Validates that every item price and quantity is strictly positive before processing

export async function createOrder(req: Request, env: Env) {
  const { items } = await req.json() as { items: { productId: string; quantity: number; price: number }[] };

  let total = 0;
  for (const item of items) {
    if (item.quantity <= 0) throw new Error(`Invalid quantity: ${item.quantity}`);
    if (item.price <= 0) throw new Error(`Invalid price for ${item.productId}: ${item.price}`);

    const product = await env.DB.prepare(
      'SELECT price FROM products WHERE id = ?'
    ).bind(item.productId).first();

    if (!product) throw new Error(`Product ${item.productId} not found`);

    // SAFE: use DB price, not client price
    total += Number(product.price) * item.quantity;
  }

  const order = await env.DB.prepare(
    'INSERT INTO orders (total, status) VALUES (?, ?) RETURNING id'
  ).bind(total, 'PENDING').first();

  return Response.json({ orderId: order.id });
}
