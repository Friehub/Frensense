// SAFE: Privilege is determined from the server-side session, never from URL params
export async function adminHandler(req: Request): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  return handleAdminDashboard(req);
}

export async function deleteUser(req: Request): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const userId = req.params.id;
  await db.prepare('DELETE FROM users WHERE id = ?').bind(userId).run();
  return new Response('Deleted');
}
