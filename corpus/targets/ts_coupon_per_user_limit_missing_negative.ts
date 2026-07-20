// SAFE: Checks the user's usage count against the coupon's per-user limit before applying

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // SAFE: check per-user usage limit
  if (coupon.per_user_limit > 0) {
    const usage = await env.DB.prepare(
      'SELECT COUNT(*) AS count FROM coupon_usage WHERE coupon_id = ? AND user_id = ?'
    ).bind(coupon.id, userId).first();

    if (usage.count >= coupon.per_user_limit) {
      throw new Error('Coupon usage limit reached for this user');
    }
  }

  const discount = calculateDiscount(coupon);
  applyToCartTotal(userId, discount);

  await env.DB.prepare(
    'INSERT INTO coupon_usage (coupon_id, user_id) VALUES (?, ?)'
  ).bind(coupon.id, userId).run();
}
