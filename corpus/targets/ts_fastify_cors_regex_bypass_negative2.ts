// SAFE: CORS origin regex is properly anchored and validates the full domain

import Fastify from 'fastify';
import cors from '@fastify/cors';

const app = Fastify();

await app.register(cors, {
  origin: /^https:\/\/[a-z0-9-]+\.example\.com$/,
});

app.get('/api/users', async (request, reply) => {
  return reply.send([{ id: 1, name: 'Alice' }]);
});

export default app;
