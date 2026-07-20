// SAFE: Checks the coupon's expiry date against the current server time

export async function applyCoupon(couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ?'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Coupon not found');

  // SAFE: check expiry date
  if (coupon.expires_at && new Date(coupon.expires_at) < new Date()) {
    throw new Error('Coupon has expired');
  }

  const discount = calculateDiscount(coupon);
  return { discount };
}
