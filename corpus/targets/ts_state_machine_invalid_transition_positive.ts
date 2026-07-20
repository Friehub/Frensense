// [frensense]
// observation: An admin endpoint sets any status value directly on the orders table without validating that the transition is legal in the order lifecycle.
// impact: An attacker with admin access can roll back a COMPLETED order to PENDING, triggering duplicate refunds, re-shipments, or workflow corruption.
// improvement: Define a whitelist of allowed transitions and validate the current state against the target state before updating.

export async function adminUpdateOrderStatus(
  orderId: string,
  newStatus: string,
  env: Env
) {
  // VULNERABLE: blindly sets any status; COMPLETED → PENDING is allowed
  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind(newStatus, orderId).run();

  if (newStatus === 'CANCELLED') {
    await issueRefund(orderId, env);
  }

  return { success: true };
}

const ALLOWED_TRANSITIONS: Record<string, string[]> = {}; // unused — the bug
