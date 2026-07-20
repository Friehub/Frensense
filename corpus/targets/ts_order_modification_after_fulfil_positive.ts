// [frensense]
// observation: An order can be modified (quantity changed, address updated, items added) after it has been fulfilled or shipped, bypassing the finality of the order lifecycle.
// impact: A customer can change the shipping address of an already-shipped order, or add items to a fulfilled order, causing logistical chaos and potential fraud.
// improvement: Check that the order is in a modifiable state (e.g., PENDING or PROCESSING) before allowing modifications.

export async function modifyOrder(orderId: string, updates: Record<string, any>, env: Env) {
  // VULNERABLE: no state check — order can be modified after fulfillment
  await env.DB.prepare(
    'UPDATE orders SET ? WHERE id = ?'
  ).bind(updates, orderId).run();

  return { modified: true };
}
