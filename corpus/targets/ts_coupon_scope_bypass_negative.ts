// SAFE: Coupon scope is verified against the cart contents before applying the discount
async function applyDiscount(cart: Cart, couponCode: string, db: DB) {
  const coupon = await db.prepare('SELECT * FROM coupons WHERE code = ?').bind(couponCode).first();
  if (!coupon || new Date() > new Date(coupon.expires_at)) {
    throw new Error('Invalid coupon');
  }

  // SAFE: calculates eligible subtotal by filtering cart items against the coupon's strict scope
  let eligibleTotal = 0;
  for (const item of cart.items) {
    if (!coupon.restricted_seller_id || item.seller_id === coupon.restricted_seller_id) {
      if (!coupon.restricted_category || item.category === coupon.restricted_category) {
        eligibleTotal += item.price * item.quantity;
      }
    }
  }

  const discountAmount = Math.min(eligibleTotal * (coupon.discount_percent / 100), coupon.max_discount);
  
  return {
    newTotal: cart.total - discountAmount,
    discountAmount
  };
}
