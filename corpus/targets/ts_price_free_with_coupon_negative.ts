// SAFE: Caps the maximum discount percentage at 99% and enforces a minimum charge of $0.50

export async function checkoutWithCoupon(userId: string, couponCode: string, env: Env) {
  const coupon = await env.DB.prepare(
    'SELECT * FROM coupons WHERE code = ? AND active = 1'
  ).bind(couponCode).first();

  if (!coupon) throw new Error('Invalid coupon');

  const cart = await env.DB.prepare(
    'SELECT total FROM carts WHERE user_id = ?'
  ).bind(userId).first();

  // SAFE: cap discount at 99% and enforce minimum charge
  const effectivePercent = Math.min(coupon.percent_off, 99);
  let finalTotal = cart.total * (1 - effectivePercent / 100);

  finalTotal = Math.max(finalTotal, 0.50); // minimum charge of $0.50

  await env.DB.prepare(
    'INSERT INTO orders (user_id, total, coupon_id, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, finalTotal, coupon.id, 'PENDING').run();
}
