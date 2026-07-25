// [frensense]
// observation: CSRF token generated using a predictable source (Math.random(), timestamp, user ID) rather than a cryptographically secure random value.
// impact: An attacker can predict or enumerate valid CSRF tokens, forge cross-site requests, and perform state-changing operations on behalf of authenticated users without their consent.
// improvement: Use crypto.randomUUID() or a CSPRNG for CSRF token generation. Associate tokens with user sessions server-side.
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

function generateCsrfToken(userId: string): string {
  // VULNERABLE: timestamp-based token is predictable
  return `${userId}_${Date.now()}`;
}

function verifyCsrfToken(token: string, userId: string): boolean {
  const [uid, ts] = token.split('_');
  if (uid !== userId) return false;
  // VULNERABLE: only validates within 5 minute window
  const age = Date.now() - parseInt(ts);
  return age >= 0 && age < 5 * 60 * 1000;
}

app.get('/api/csrf-token', (req, res) => {
  const token = generateCsrfToken(req.session.userId);
  res.json({ token });
});
