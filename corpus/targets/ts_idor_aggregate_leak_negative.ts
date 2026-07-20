// SAFE: Aggregate queries are scoped to the authenticated user's data
export async function getDashboardStats(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const stats = {
    totalOrders: await db.prepare('SELECT COUNT(*) as count FROM orders WHERE user_id = ?').bind(session.userId).first(),
    totalSpent: await db.prepare('SELECT SUM(amount) as total FROM transactions WHERE user_id = ?').bind(session.userId).first(),
    averageOrderValue: await db.prepare('SELECT AVG(total) as avg FROM orders WHERE user_id = ?').bind(session.userId).first(),
    recentOrders: await db.prepare('SELECT COUNT(*) as count FROM orders WHERE user_id = ? AND created_at > ?').bind(session.userId, Date.now() - 86400000).first()
  };
  return new Response(JSON.stringify(stats));
}
