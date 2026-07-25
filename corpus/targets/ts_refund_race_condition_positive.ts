// [frensense]
// observation: The refund eligibility check and the refund processing are two separate database operations with no transaction or lock, creating a TOCTOU race window.
// impact: An attacker can send concurrent refund requests; both pass the eligibility check before either processes, resulting in a double refund.
// improvement: Wrap the eligibility check and the refund in a database transaction, or use an atomic UPDATE with a guard condition.
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

export async function processRefund(orderId: string, env: Env) {
  // VULNERABLE: separate read and write — race condition
  const order = await env.DB.prepare(
    'SELECT status FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order || order.status !== 'DELIVERED') {
    throw new Error('Order is not eligible for refund');
  }

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('REFUND_PENDING', orderId).run();

  await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
  });
}
