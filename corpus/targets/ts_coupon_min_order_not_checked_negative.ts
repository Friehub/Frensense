// SAFE: Validates the cart total against the coupon's minimum order amount before applying

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  const cart = await env.DB.prepare(
    'SELECT total FROM carts WHERE user_id = ?'
  ).bind(userId).first();

  // SAFE: check minimum order amount
  if (coupon.min_order_amount && Number(cart.total) < Number(coupon.min_order_amount)) {
    throw new Error(
      `Minimum order amount of ${coupon.min_order_amount} not met`
    );
  }

  const discount = Math.min(cart.total * (coupon.percent_off / 100), coupon.max_discount || Infinity);
  const finalTotal = cart.total - discount;

  await env.DB.prepare(
    'UPDATE carts SET total = ?, applied_coupon = ? WHERE user_id = ?'
  ).bind(finalTotal, couponCode, userId).run();

  return { finalTotal };
}
