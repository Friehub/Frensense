// [frensense]
// observation: A coupon's global usage limit (max redemptions) is not checked before applying it, allowing far more redemptions than intended.
// impact: An attacker can redeem a coupon designed for 100 uses thousands of times, causing massive unearned discounts and financial loss.
// improvement: Check the total usage count against the coupon's max_redemptions before allowing the discount.

export async function redeemCoupon(couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // VULNERABLE: no check on global usage limit
  const discount = calculateDiscount(coupon);
  await env.DB.prepare(
    'INSERT INTO coupon_usage (coupon_id) VALUES (?)'
  ).bind(coupon.id).run();

  return { discount };
}
