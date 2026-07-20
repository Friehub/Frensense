// [frensense]
// observation: A coupon restricted to a specific product category is applied to items outside that category, bypassing the category restriction.
// impact: Users can use a coupon meant for low-margin clearance items on high-margin premium items, reducing profits where it hurts most.
// improvement: Filter the cart items by the coupon's restricted category before applying the discount.

export async function applyCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  const cart = await env.DB.prepare(
    'SELECT total FROM carts WHERE user_id = ?'
  ).bind(userId).first();

  // VULNERABLE: coupon with restricted_category applied to full cart total
  let discount = cart.total * (coupon.percent_off / 100);
  if (coupon.max_discount) discount = Math.min(discount, coupon.max_discount);

  const finalTotal = cart.total - discount;
  await env.DB.prepare(
    'UPDATE carts SET total = ?, applied_coupon = ? WHERE user_id = ?'
  ).bind(finalTotal, couponCode, userId).run();

  return { finalTotal };
}
