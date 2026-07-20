// SAFE: Filters cart items by the coupon's restricted category before computing the eligible total

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  // SAFE: calculate eligible total from restricted category only
  let eligibleTotal = 0;

  const cartItems = await env.DB.prepare(
    'SELECT ci.price, ci.quantity, p.category_id FROM cart_items ci JOIN products p ON p.id = ci.product_id WHERE ci.cart_user_id = ?'
  ).bind(userId).all();

  for (const item of cartItems) {
    if (item.category_id === coupon.restricted_category_id) {
      eligibleTotal += Number(item.price) * item.quantity;
    }
  }

  const discount = Math.min(eligibleTotal * (coupon.percent_off / 100), coupon.max_discount || Infinity);
  const finalTotal = Math.max(0, (cartItems.reduce((s, i) => s + Number(i.price) * i.quantity, 0)) - discount);

  await env.DB.prepare(
    'UPDATE carts SET total = ? WHERE user_id = ?'
  ).bind(finalTotal, userId).run();

  return { finalTotal, discount };
}
