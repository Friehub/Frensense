// [frensense]
// observation: A resource sharing endpoint allows a user to share any resource ID with another user without verifying that they own the resource.
// impact: An attacker can make anyone's private resources publicly accessible or shared with attacker-controlled accounts, bypassing access controls.
// improvement: Verify the current user owns the resource before allowing sharing operations.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function shareDocument(req: Request, db: DB): Promise<Response> {
  const { docId, shareWithEmail } = await req.json();
  const targetUser = await db.prepare('SELECT id FROM users WHERE email = ?').bind(shareWithEmail).first();
  if (!targetUser) return new Response('User not found', { status: 404 });
  await db.prepare('INSERT INTO document_shares (document_id, user_id) VALUES (?, ?)').bind(docId, targetUser.id).run();
  return new Response(JSON.stringify({ shared: true }));
}

export async function makePublic(req: Request, db: DB): Promise<Response> {
  const { docId } = req.body;
  await db.prepare('UPDATE documents SET is_public = 1 WHERE id = ?').bind(docId).run();
  return new Response('Made public');
}
