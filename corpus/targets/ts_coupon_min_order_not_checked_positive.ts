// [frensense]
// observation: A coupon's minimum order amount requirement is not validated against the cart total, allowing discounts on orders below the threshold.
// impact: Users can apply coupons intended for large orders to small purchases, eroding profit margins on low-value transactions.
// improvement: Compare the cart total against the coupon's min_order_amount before applying the discount.

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  const cart = await env.DB.prepare(
    'SELECT total FROM carts WHERE user_id = ?'
  ).bind(userId).first();

  // VULNERABLE: no check that cart total meets the minimum order requirement
  const discount = Math.min(cart.total * (coupon.percent_off / 100), coupon.max_discount || Infinity);
  const finalTotal = cart.total - discount;

  await env.DB.prepare(
    'UPDATE carts SET total = ?, applied_coupon = ? WHERE user_id = ?'
  ).bind(finalTotal, couponCode, userId).run();

  return { finalTotal };
}
