// [frensense]
// observation: The session ID from the pre-login cookie is reused after authentication; regenerateSession is never called.
// impact: An attacker can fixate a session ID, trick the victim into logging in with that ID, and then hijack the authenticated session.
// improvement: Call req.session.regenerate() or similar session rotation after successful login.
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

import session from 'express-session';

export async function login(req: Request, res: Response, db: DB): Promise<void> {
  const { username, password } = req.body;
  const user = await db.prepare('SELECT * FROM users WHERE username = ?').bind(username).first();
  if (!user || user.password !== password) {
    res.status(401).json({ error: 'Invalid credentials' });
    return;
  }
  req.session.userId = user.id;
  req.session.role = user.role;
  res.json({ success: true });
}
