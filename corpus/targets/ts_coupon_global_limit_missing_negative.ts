// SAFE: Checks the global usage count against max_redemptions before allowing the discount

export async function redeemCoupon(couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // SAFE: check global usage limit
  if (coupon.max_redemptions > 0) {
    const usage = await env.DB.prepare(
      'SELECT COUNT(*) AS count FROM coupon_usage WHERE coupon_id = ?'
    ).bind(coupon.id).first();

    if (usage.count >= coupon.max_redemptions) {
      throw new Error('Coupon has reached its maximum number of redemptions');
    }
  }

  const discount = calculateDiscount(coupon);
  await env.DB.prepare(
    'INSERT INTO coupon_usage (coupon_id) VALUES (?)'
  ).bind(coupon.id).run();

  return { discount };
}
