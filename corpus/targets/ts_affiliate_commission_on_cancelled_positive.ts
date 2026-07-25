// [frensense]
// observation: Affiliate commission is paid immediately when the order is placed, even if the order is later cancelled, and the commission is never reversed.
// impact: A fraudulent affiliate can refer orders, collect commission, then cancel the orders, earning money without bringing any real sales.
// improvement: Defer commission payment until the order is delivered/completed, or reverse commission on cancellation.
// cwe: CWE-754
// cvss: 6.5
// owasp: 
// severity: Medium

export async function processOrder(userId: string, affiliateCode: string, total: number, env: Env) {
  const order = await env.DB.prepare(
    'INSERT INTO orders (user_id, total, status, affiliate_code) VALUES (?, ?, ?, ?) RETURNING id'
  ).bind(userId, total, 'PENDING', affiliateCode).first();

  if (affiliateCode) {
    // VULNERABLE: pays commission immediately, even on cancellable orders
    const commission = total * 0.1;
    const affiliate = await env.DB.prepare(
      'SELECT id FROM affiliates WHERE code = ?'
    ).bind(affiliateCode).first();

    if (affiliate) {
      await env.DB.prepare(
        'UPDATE affiliates SET balance = balance + ? WHERE id = ?'
      ).bind(commission, affiliate.id).run();

      await env.DB.prepare(
        'INSERT INTO affiliate_transactions (affiliate_id, order_id, amount, type) VALUES (?, ?, ?, ?)'
      ).bind(affiliate.id, order.id, commission, 'commission').run();
    }
  }

  return { orderId: order.id };
}
