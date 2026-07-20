// SAFE: Only allowed fields are whitelisted for updates
const ALLOWED_UPDATE_FIELDS = ['name', 'email', 'avatar_url', 'bio', 'timezone'];

export async function updateProfile(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const body = await req.json();
  const updates: Record<string, any> = {};
  for (const field of ALLOWED_UPDATE_FIELDS) {
    if (body[field] !== undefined) updates[field] = body[field];
  }
  if (Object.keys(updates).length === 0) return new Response('No valid fields', { status: 400 });
  const setClauses = Object.keys(updates).map(k => `${k} = ?`).join(', ');
  await db.prepare(`UPDATE users SET ${setClauses} WHERE id = ?`).bind(...Object.values(updates), session.userId).run();
  return new Response(JSON.stringify({ updated: true }));
}
