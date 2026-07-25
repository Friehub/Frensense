// [frensense]
// observation: Fastify CORS origin option uses a regex pattern with a wildcard that can match unintended domains, allowing cross-origin access from malicious sites.
// impact: An attacker can craft an origin header like 'evil-attacker.com' and bypass CORS restrictions, enabling data exfiltration via XSS.
// improvement: Use an exact origin list or an anchored regex with start and end boundaries instead of a loosely matching pattern.
// cwe: CWE-942
// cvss: 8.8
// owasp: A05:2021
// severity: High

import Fastify from 'fastify';
import cors from '@fastify/cors';

const app = Fastify();

await app.register(cors, {
  origin: /.*\.example\.com/,
});

app.get('/api/users', async (request, reply) => {
  return reply.send([{ id: 1, name: 'Alice' }]);
});

export default app;
