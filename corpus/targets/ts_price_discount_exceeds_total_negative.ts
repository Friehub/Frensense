// SAFE: Clamps the final total to zero so it can never go negative

export async function applyPromoCode(cart: { total: number }, code: string, env: Env) {
  const promo = await env.DB.prepare(
    'SELECT * FROM promotions WHERE code = ? AND active = 1'
  ).bind(code).first();

  if (!promo) throw new Error('Invalid promo code');

  const discount = Math.min(
    cart.total * (promo.percent_off / 100),
    promo.max_discount || Infinity
  );

  // SAFE: floor at zero
  const finalTotal = Math.max(0, cart.total - discount);

  return { finalTotal, discount };
}
