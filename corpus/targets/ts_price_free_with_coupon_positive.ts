// [frensense]
// observation: A 100% discount coupon is accepted without capping the maximum discount percentage, allowing the entire order total to become zero.
// impact: An attacker can obtain items for free by applying a 100%-off coupon, causing the merchant to bear the full cost of goods and shipping.
// improvement: Enforce a maximum discount percentage (e.g., 99%) or require a minimum charge amount even when a coupon is applied.

export async function checkoutWithCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  const cart = await env.DB.prepare(
    'SELECT total FROM carts WHERE user_id = ?'
  ).bind(userId).first();

  // VULNERABLE: 100% discount results in a $0 order
  let finalTotal = cart.total * (1 - coupon.percent_off / 100);

  await env.DB.prepare(
    'INSERT INTO orders (user_id, total, coupon_id, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, finalTotal, coupon.id, 'PENDING').run();
}
