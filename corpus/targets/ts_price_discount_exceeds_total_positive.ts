// [frensense]
// observation: A percentage-based discount is subtracted from the total without enforcing a floor at zero, allowing the final price to become negative when the discount exceeds the total.
// impact: An attacker can use a high-value coupon or promotion to make the total negative, potentially earning money from a purchase or offsetting other charges.
// improvement: Clamp the discounted total to at least zero using Math.max(0, total - discount).

export async function applyPromoCode(cart: { total: number }, code: string, env: Env) {
  const promo = await env.DB.prepare(
    'SELECT * FROM promotions WHERE code = ? AND active = 1'
  ).bind(code).first();

  if (!promo) throw new Error('Invalid promo code');

  const discount = Math.min(
    cart.total * (promo.percent_off / 100),
    promo.max_discount || Infinity
  );

  // VULNERABLE: discount can exceed total, making it negative
  const finalTotal = cart.total - discount;

  return { finalTotal, discount };
}
