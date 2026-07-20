// SAFE: Server-side price recalculation from database prices ensures the client cannot manipulate the total

export async function checkout(req: Request, env: Env) {
  const { items } = await req.json() as { items: CartItem[] };

  // SAFE: recalculate total server-side from stored prices
  let total = 0;
  for (const item of items) {
    const product = await env.DB.prepare(
      'SELECT price FROM products WHERE id = ?'
    ).bind(item.productId).first();

    if (!product) throw new Error(`Product ${item.productId} not found`);
    total += Number(product.price) * item.quantity;
  }

  const paymentIntent = await env.STRIPE.paymentIntents.create({
    amount: Math.round(total * 100),
    currency: 'usd',
  });

  const order = await env.DB.prepare(
    'INSERT INTO orders (total, stripe_pi, status) VALUES (?, ?, ?) RETURNING id'
  ).bind(total, paymentIntent.id, 'PENDING').first();

  return Response.json({ orderId: order.id, clientSecret: paymentIntent.client_secret });
}
