// SAFE: API keys are scoped to specific permissions; admin and user keys are separate
const CONFIG = { admin: { key: process.env.ADMIN_API_KEY!, scope: 'admin:all' }, read: { key: process.env.READ_API_KEY!, scope: 'read:users' } };

export async function apiHandler(req: Request): Promise<Response> {
  const key = req.headers.get('x-api-key');
  const config = Object.values(CONFIG).find(c => c.key === key);
  if (!config) return new Response('Unauthorized', { status: 401 });
  req.scope = config.scope;
  return handleRoute(req);
}

export async function deleteUser(userId: string): Promise<Response> {
  return fetch(`https://api.example.com/admin/users/${userId}`, {
    method: 'DELETE',
    headers: { 'X-Api-Key': CONFIG.admin.key }
  });
}
