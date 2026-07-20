// [frensense]
// observation: A coupon with a per-user usage limit is accepted without checking how many times the current user has already used it, allowing unlimited reuse.
// impact: A user can apply the same coupon code repeatedly, receiving the discount on every order instead of the intended one-time use.
// improvement: Look up the user's prior usage count for the coupon and reject if they have exceeded the per-user limit.

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // VULNERABLE: does not check how many times this user has used the coupon
  const discount = calculateDiscount(coupon);
  applyToCartTotal(userId, discount);

  await env.DB.prepare(
    'INSERT INTO coupon_usage (coupon_id, user_id) VALUES (?, ?)'
  ).bind(coupon.id, userId).run();
}
