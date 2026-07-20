// [frensense]
// observation: A coupon restricted to a specific user is accepted for any user's order, bypassing the user ownership restriction.
// impact: Users can share exclusive personal discount codes with others, allowing unauthorized users to benefit from targeted promotions.
// improvement: Verify that the authenticated user matches the coupon's owner_user_id before applying the discount.

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // VULNERABLE: does not check if coupon is restricted to a different user
  const discount = calculateDiscount(coupon);
  await applyToCart(userId, discount);

  return { discount };
}
