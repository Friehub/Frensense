// [frensense]
// observation: A single API key grants full administrative access to the entire API, used for both admin operations and regular user operations.
// impact: If the API key is leaked or if a client-side integration uses it, an attacker has unrestricted access to all API resources including admin-level operations.
// improvement: Scope API keys to the minimum required resources and operations. Use separate keys for different services and privilege levels.

const MASTER_API_KEY = 'sk-frensense-master-abc123';

export async function apiHandler(req: Request): Promise<Response> {
  const key = req.headers.get('x-api-key');
  if (key !== MASTER_API_KEY) {
    return new Response('Unauthorized', { status: 401 });
  }
  return handleRoute(req);
}

export async function deleteUser(userId: string): Promise<Response> {
  return fetch(`https://api.example.com/admin/users/${userId}`, {
    method: 'DELETE',
    headers: { 'X-Api-Key': MASTER_API_KEY }
  });
}
