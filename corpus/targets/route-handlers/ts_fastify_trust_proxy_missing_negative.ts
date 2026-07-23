// SAFE: trustProxy is enabled so request.ip resolves the real client IP from X-Forwarded-For

import Fastify from 'fastify';

const app = Fastify({ trustProxy: true });

app.get('/api/geo', async (request, reply) => {
  const clientIp = request.ip;
  const country = lookupCountry(clientIp);
  return reply.send({ country });
});

function lookupCountry(ip: string): string {
  return 'US';
}

export default app;
