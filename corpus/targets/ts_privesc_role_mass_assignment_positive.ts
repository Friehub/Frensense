// [frensense]
// observation: The user profile update endpoint accepts any fields from the request body and applies them directly to the database, including the role field.
// impact: An attacker can escalate their privileges by sending { "role": "admin" } in the profile update request, gaining administrative access.
// improvement: Whitelist the fields that can be updated by the user. Never allow the role or privilege fields to be set through mass assignment.

export async function updateProfile(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const updates = await req.json();
  const setClauses = Object.entries(updates).map(([key]) => `${key} = ?`).join(', ');
  const values = Object.values(updates);
  await db.prepare(`UPDATE users SET ${setClauses} WHERE id = ?`).bind(...values, session.userId).run();
  return new Response(JSON.stringify({ updated: true }));
}

export async function patchUser(req: Request, db: DB): Promise<Response> {
  const userId = req.params.id;
  const fields = req.body;
  await db.prepare(`UPDATE users SET ${Object.keys(fields).map(k => `${k} = ?`).join(', ')} WHERE id = ?`).bind(...Object.values(fields), userId).run();
  return new Response('Updated');
}
