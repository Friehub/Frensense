// [frensense]
// observation: A promotional code or discount is applied globally without verifying its restricted scope (e.g., restricted to a specific seller or item category).
// impact: Users apply a coupon intended for a low-value item to a high-value purchase, circumventing business logic and causing financial loss.
// improvement: Query the coupon restrictions and intersect them with the cart items before calculating the discount.

async function applyDiscount(cart: Cart, couponCode: string, db: DB) {
  const coupon = await db.prepare('SELECT * FROM coupons WHERE code = ?').bind(couponCode).first();
  if (!coupon || new Date() > new Date(coupon.expires_at)) {
    throw new Error('Invalid coupon');
  }

  // VULNERABLE: coupon applied to the entire cart total without checking seller/category scope
  const discountAmount = Math.min(cart.total * (coupon.discount_percent / 100), coupon.max_discount);
  
  return {
    newTotal: cart.total - discountAmount,
    discountAmount
  };
}
