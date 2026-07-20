// [frensense]
// observation: A payment webhook handler transitions the order to COMPLETED without checking the current state, allowing replay of the webhook event to create multiple completed orders.
// impact: An attacker can replay a webhook notification to transition an already-completed order through the COMPLETED state again, potentially triggering duplicate fulfillments or payouts.
// improvement: Verify that the order's current state allows the transition (e.g., only PENDING or AWAITING_CONFIRMATION can become COMPLETED) and use idempotency keys.

export async function handleStripeWebhook(request: Request, env: Env) {
  const body = await request.json();
  const event = body as StripeEvent;

  if (event.type === 'payment_intent.succeeded') {
    const paymentIntent = event.data.object as PaymentIntent;
    const orderId = paymentIntent.metadata.orderId;

    // VULNERABLE: directly transitions order to COMPLETED without state validation
    await env.DB.prepare(
      'UPDATE orders SET status = ? WHERE id = ?'
    ).bind('COMPLETED', orderId).run();

    await env.DB.prepare(
      'UPDATE payments SET status = ? WHERE stripe_pi = ?'
    ).bind('completed', paymentIntent.id).run();
  }

  return new Response('ok');
}
