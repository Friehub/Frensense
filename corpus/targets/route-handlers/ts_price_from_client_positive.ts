// [frensense]
// observation: The total price is taken directly from the request body rather than calculated server-side from the cart items' stored prices and quantities.
// impact: A user can set the total to $0.01 or even a negative amount by manipulating the request body, purchasing items for far below their intended price.
// improvement: Always recalculate the total server-side: query the current price from the database for each item and multiply by the requested quantity.

export async function checkout(req: Request, env: Env) {
  const { items, total } = await req.json() as {
    items: CartItem[];
    total: number;
  };

  // VULNERABLE: trusts the client-provided total
  const paymentIntent = await env.STRIPE.paymentIntents.create({
    amount: Math.round(total * 100),
    currency: 'usd',
  });

  const order = await env.DB.prepare(
    'INSERT INTO orders (total, stripe_pi, status) VALUES (?, ?, ?) RETURNING id'
  ).bind(total, paymentIntent.id, 'PENDING').first();

  return Response.json({ orderId: order.id, clientSecret: paymentIntent.client_secret });
}
