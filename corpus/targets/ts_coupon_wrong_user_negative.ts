// SAFE: Verifies that the coupon is either unrestricted or belongs to the current user

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // SAFE: check coupon ownership
  if (coupon.owner_user_id && coupon.owner_user_id !== userId) {
    throw new Error('This coupon is not valid for your account');
  }

  const discount = calculateDiscount(coupon);
  await applyToCart(userId, discount);

  return { discount };
}
