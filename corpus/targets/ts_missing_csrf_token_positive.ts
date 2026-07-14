// [frensense]
// observation = "A state-changing API route (POST/PUT/DELETE) parses cookies for authentication but fails to validate a CSRF token."
// impact = "An attacker can host a malicious site that submits state-changing requests to the API using the victim's ambient cookie credentials (CSRF)."
// improvement = "Require a custom request header (e.g., X-CSRF-Token) or migrate to Bearer token authentication instead of ambient cookies."

app.post('/api/settings/update', async (req, res) => {
  // VULNERABLE: Relies strictly on a cookie for auth, with no CSRF guard
  const sessionId = req.cookies['session_id'];
  const user = await validateSession(sessionId);
  if (!user) return res.status(401).send('Unauthorized');

  await db.updateSettings(user.id, req.body.settings);
  res.send('Success');
});
