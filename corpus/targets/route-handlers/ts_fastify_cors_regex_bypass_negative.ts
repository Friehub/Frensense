// SAFE: CORS origin uses an allow list of exact strings so only permitted origins are accepted

import Fastify from 'fastify';
import cors from '@fastify/cors';

const app = Fastify();

await app.register(cors, {
  origin: ['https://app.example.com', 'https://admin.example.com'],
});

app.get('/api/users', async (request, reply) => {
  return reply.send([{ id: 1, name: 'Alice' }]);
});

export default app;
