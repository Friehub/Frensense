// SAFE: Adds the bundle as a single line item with the fixed bundle price instead of individual components

export async function addBundleToCart(userId: string, bundleId: string, env: Env) {
  const bundle = await env.DB.prepare(
    'SELECT * FROM bundles WHERE id = ? AND active = 1'
  ).bind(bundleId).first();

  if (!bundle) throw new Error('Bundle not found');

  // SAFE: add bundle as a single line item with fixed bundle price
  await env.DB.prepare(
    'INSERT INTO cart_items (user_id, bundle_id, quantity, price, is_bundle) VALUES (?, ?, ?, ?, ?)'
  ).bind(userId, bundleId, 1, bundle.bundle_price, 1).run();

  return { added: true };
}
