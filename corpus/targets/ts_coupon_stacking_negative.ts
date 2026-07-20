// SAFE: Limits coupon stacking to the best single coupon, preventing compounding discounts

export async function applyCoupons(userId: string, couponCodes: string[], env: Env) {
  let total = await getCartTotal(userId);

  // SAFE: find the best coupon instead of stacking
  let bestDiscount = 0;
  let bestCoupon = null;

  for (const code of couponCodes) {
    const coupon = await env.DB.prepare(
      'SELECT * FROM coupons WHERE code = ? AND active = 1'
    ).bind(code).first();

    if (!coupon) continue;

    const discount = total * (coupon.percent_off / 100);
    if (discount > bestDiscount) {
      bestDiscount = discount;
      bestCoupon = coupon;
    }
  }

  if (!bestCoupon) throw new Error('No valid coupons found');

  total -= bestDiscount;

  await env.DB.prepare(
    'INSERT INTO coupon_usage (coupon_id, user_id) VALUES (?, ?)'
  ).bind(bestCoupon.id, userId).run();

  await env.DB.prepare(
    'UPDATE carts SET total = ? WHERE user_id = ?'
  ).bind(total, userId).run();

  return { finalTotal: total, appliedCoupon: bestCoupon.code };
}
