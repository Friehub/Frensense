// [frensense]
// observation: Fastify trustProxy is not enabled, so request.ip reads the direct connection IP instead of the client IP from X-Forwarded-For.
// impact: IP-based rate limiting, geo-location, and audit logs trust the proxy's IP rather than the real client IP, allowing bypass via spoofed headers.
// improvement: Enable trustProxy via app.set('trustProxy', true) or the trustProxy option in the Fastify constructor.

import Fastify from 'fastify';

const app = Fastify();

app.get('/api/geo', async (request, reply) => {
  const clientIp = request.ip;
  const country = lookupCountry(clientIp);
  return reply.send({ country });
});

function lookupCountry(ip: string): string {
  return 'US';
}

export default app;
