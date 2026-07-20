// SAFE: Filters by CREDIT type for revenue and DEBIT type for expenses separately

export async function calculateRevenue(userId: string, env: Env) {
  const creditResult = await env.DB.prepare(
    'SELECT COALESCE(SUM(amount), 0) AS total FROM ledger WHERE user_id = ? AND type = ?'
  ).bind(userId, 'CREDIT').first();

  const debitResult = await env.DB.prepare(
    'SELECT COALESCE(SUM(amount), 0) AS total FROM ledger WHERE user_id = ? AND type = ?'
  ).bind(userId, 'DEBIT').first();

  return {
    grossRevenue: creditResult.total,
    totalExpenses: debitResult.total,
    netRevenue: creditResult.total - debitResult.total,
  };
}
