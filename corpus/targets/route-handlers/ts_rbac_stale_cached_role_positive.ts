// [frensense]
// observation: User roles are cached in the session at login time and never refreshed from the database during the session lifetime.
// impact: If an admin demotes a user or suspends their account, the cached role remains active until the session expires, allowing the user to continue unauthorized operations.
// improvement: Refresh role/permission data from the database on every request or at short intervals, especially for sensitive operations.

export async function dashboardHandler(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  return handleAdminDashboard(req, db);
}

export async function manageUsers(req: Request): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  return handleUserManagement(req);
}
