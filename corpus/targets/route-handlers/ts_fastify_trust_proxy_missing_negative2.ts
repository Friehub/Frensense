// SAFE: trustProxy is configured with a specific number of trusted proxy hops

import Fastify from 'fastify';

const app = Fastify();

app.set('trustProxy', 1);

app.get('/api/geo', async (request, reply) => {
  const clientIp = request.ip;
  const country = lookupCountry(clientIp);
  return reply.send({ country });
});

function lookupCountry(ip: string): string {
  return 'US';
}

export default app;
