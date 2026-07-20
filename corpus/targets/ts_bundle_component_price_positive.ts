// [frensense]
// observation: A bundle's component items can be priced individually and the total can be manipulated by extracting individual components from the bundle.
// impact: A user can add a bundle to the cart, manipulate the individual component prices, or remove components at a different price ratio than intended, paying less than the bundle price.
// improvement: Apply the bundle price as a fixed total and prevent individual component price manipulation.

export async function addBundleToCart(userId: string, bundleId: string, env: Env) {
  const bundle = await env.DB.prepare(
    'SELECT * FROM bundles WHERE id = ? AND active = 1'
  ).bind(bundleId).first();

  if (!bundle) throw new Error('Bundle not found');

  const components = await env.DB.prepare(
    'SELECT * FROM bundle_components WHERE bundle_id = ?'
  ).bind(bundleId).all();

  // VULNERABLE: adds components individually, allowing the user to
  // later manipulate their individual prices or remove some
  for (const component of components) {
    await env.DB.prepare(
      'INSERT INTO cart_items (user_id, product_id, quantity, price) VALUES (?, ?, ?, ?)'
    ).bind(userId, component.product_id, component.quantity, component.individual_price).run();
  }

  return { added: true };
}
