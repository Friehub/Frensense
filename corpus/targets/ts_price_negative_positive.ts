// [frensense]
// observation: The price or quantity is accepted from the client without validating that the value is positive, allowing negative amounts to be submitted.
// impact: An attacker can set a negative price for an item, causing the total to decrease or go negative, effectively stealing from the merchant.
// improvement: Validate that all prices and quantities are greater than zero on the server side before processing the order.

export async function createOrder(req: Request, env: Env) {
  const { items } = await req.json() as { items: { productId: string; quantity: number; price: number }[] };

  // VULNERABLE: negative quantity or price is not rejected
  let total = 0;
  for (const item of items) {
    total += item.price * item.quantity;
  }

  if (total <= 0) throw new Error('Invalid total');

  const order = await env.DB.prepare(
    'INSERT INTO orders (total, status) VALUES (?, ?) RETURNING id'
  ).bind(total, 'PENDING').first();

  return Response.json({ orderId: order.id });
}
