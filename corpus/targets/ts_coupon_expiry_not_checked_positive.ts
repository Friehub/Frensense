// [frensense]
// observation: The coupon's expiration date is not compared against the current date, allowing expired coupons to be accepted.
// impact: Users can indefinitely use expired promotional codes, continuing to receive discounts long after the campaign has ended.
// improvement: Compare the coupon's expiration date with the current server time before applying the discount.

export async function applyCoupon(couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ?'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Coupon not found');

  // VULNERABLE: no expiry date check
  const discount = calculateDiscount(coupon);
  return { discount };
}
