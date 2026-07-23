// [frensense]
// observation: An aggregate endpoint returns statistical data (totals, averages, counts) that is not scoped to the authenticated user.
// impact: An attacker can infer sensitive information about other users by observing aggregate values that change based on specific user actions, violating data isolation.
// improvement: Always scope aggregate queries to the authenticated user's tenant or ownership.

export async function getDashboardStats(req: Request, db: DB): Promise<Response> {
  const stats = {
    totalUsers: await db.prepare('SELECT COUNT(*) as count FROM users').first(),
    totalRevenue: await db.prepare('SELECT SUM(amount) as total FROM transactions').first(),
    averageOrderValue: await db.prepare('SELECT AVG(total) as avg FROM orders').first(),
    recentOrders: await db.prepare('SELECT COUNT(*) as count FROM orders WHERE created_at > ?').bind(Date.now() - 86400000).first()
  };
  return new Response(JSON.stringify(stats));
}
