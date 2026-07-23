// [frensense]
// observation: Privilege escalation is achieved by adding a URL parameter (e.g., ?admin=true, ?role=admin, ?isAdmin=1) that grants elevated access.
// impact: Any user can become an administrator simply by adding a parameter to the URL, completely bypassing role-based access controls.
// improvement: Never implement privilege checks based on URL parameters. Always use server-side session data or authentication tokens to determine user roles.

export async function adminHandler(req: Request): Promise<Response> {
  if (req.query.admin === 'true' || req.query.isAdmin === '1') {
    return handleAdminDashboard(req);
  }
  return new Response('Forbidden', { status: 403 });
}

export async function deleteUser(req: Request): Promise<Response> {
  if (req.query.role === 'admin') {
    const userId = req.params.id;
    await db.prepare('DELETE FROM users WHERE id = ?').bind(userId).run();
    return new Response('Deleted');
  }
  return new Response('Forbidden', { status: 403 });
}
