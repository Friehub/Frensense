// SAFE: Uses explicit field-by-field update pattern
export async function updateProfile(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { name, email, avatar_url, bio, timezone } = await req.json();
  await db.prepare('UPDATE users SET name = COALESCE(?, name), email = COALESCE(?, email), avatar_url = COALESCE(?, avatar_url), bio = COALESCE(?, bio), timezone = COALESCE(?, timezone) WHERE id = ?')
    .bind(name, email, avatar_url, bio, timezone, session.userId).run();
  return new Response(JSON.stringify({ updated: true }));
}
