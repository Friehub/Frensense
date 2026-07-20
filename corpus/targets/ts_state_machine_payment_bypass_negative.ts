// SAFE: Validates the order's current state and uses an atomic WHERE guard to prevent duplicate transitions

export async function handleStripeWebhook(request: Request, env: Env) {
  const body = await request.json();
  const event = body as StripeEvent;

  if (event.type === 'payment_intent.succeeded') {
    const paymentIntent = event.data.object as PaymentIntent;
    const orderId = paymentIntent.metadata.orderId;

    // SAFE: only transition if current status is PENDING or AWAITING_CONFIRMATION
    const validStates = ['PENDING', 'AWAITING_CONFIRMATION'];

    const result = await env.DB.prepare(
      'UPDATE orders SET status = ? WHERE id = ? AND status IN (' +
      validStates.map(() => '?').join(',') + ')'
    ).bind('COMPLETED', orderId, ...validStates).run();

    if (result.meta.changes === 0) {
      console.warn('Ignored duplicate or invalid state transition for order', orderId);
      return new Response('ignored', { status: 200 });
    }

    await env.DB.prepare(
      'UPDATE payments SET status = ? WHERE stripe_pi = ?'
    ).bind('completed', paymentIntent.id).run();
  }

  return new Response('ok');
}
