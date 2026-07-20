// [frensense]
// observation: Multiple coupons can be stacked on a single order without enforcing a stacking limit, allowing discounts to compound beyond what was intended.
// impact: A user can combine multiple high-value coupons to reduce the order total to near zero, causing the merchant to bear the full cost.
// improvement: Enforce a maximum number of coupons per order, or apply only the best coupon instead of stacking.

export async function applyCoupons(userId: string, couponCodes: string[], env: Env) {
  let total = await getCartTotal(userId);

  // VULNERABLE: applies all coupons without limit or stacking restriction
  for (const code of couponCodes) {
    const coupon = await env.DB.prepare(
      'SELECT * FROM coupons WHERE code = ? AND active = 1'
    ).bind(code).first();

    if (!coupon) continue;

    const discount = total * (coupon.percent_off / 100);
    total -= discount;

    await env.DB.prepare(
      'INSERT INTO coupon_usage (coupon_id, user_id) VALUES (?, ?)'
    ).bind(coupon.id, userId).run();
  }

  await env.DB.prepare(
    'UPDATE carts SET total = ? WHERE user_id = ?'
  ).bind(total, userId).run();

  return { finalTotal: total };
}
