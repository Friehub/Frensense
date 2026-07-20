// SAFE: Admin aggregations are protected by admin role check
export async function getAdminDashboardStats(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const stats = {
    totalUsers: await db.prepare('SELECT COUNT(*) as count FROM users').first(),
    totalRevenue: await db.prepare('SELECT SUM(amount) as total FROM transactions WHERE status = ?').bind('completed').first()
  };
  return new Response(JSON.stringify(stats));
}
