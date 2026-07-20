// SAFE: Role is fetched from the database on every request
export async function dashboardHandler(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const user = await db.prepare('SELECT role, status FROM users WHERE id = ?').bind(session.userId).first();
  if (!user || user.role !== 'admin' || user.status !== 'active') return new Response('Forbidden', { status: 403 });
  return handleAdminDashboard(req, db);
}

export async function manageUsers(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const user = await db.prepare('SELECT role FROM users WHERE id = ?').bind(session.userId).first();
  if (!user || user.role !== 'admin') return new Response('Forbidden', { status: 403 });
  return handleUserManagement(req);
}
