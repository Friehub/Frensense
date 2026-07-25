// [frensense]
// observation: A revenue aggregation query sums both DEBIT and CREDIT ledger entries, producing an inflated total that does not reflect actual revenue.
// impact: Financial reports show incorrect revenue figures, potentially leading to overpayment of taxes, dividends, or royalties, and masking losses.
// improvement: Filter the aggregation to only CREDIT entries for revenue calculations, or use separate DEBIT and CREDIT sums.
// cwe: CWE-841
// cvss: 7.5
// owasp: 
// severity: High

export async function calculateRevenue(userId: string, env: Env) {
  // VULNERABLE: sums all ledger entries regardless of type, double-counting debits as revenue
  const result = await env.DB.prepare(
    'SELECT COALESCE(SUM(amount), 0) AS total FROM ledger WHERE user_id = ?'
  ).bind(userId).first();

  return { revenue: result.total };
}
