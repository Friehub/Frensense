// SAFE: Authentication hook is registered globally via addHook so all routes are protected

import Fastify from 'fastify';
import type { FastifyRequest, FastifyReply } from 'fastify';

const app = Fastify();

app.addHook('preHandler', async (request: FastifyRequest, reply: FastifyReply) => {
  const token = request.headers.authorization;
  if (!token || !token.startsWith('Bearer ')) {
    return reply.status(401).send({ error: 'Unauthorized' });
  }
});

app.get('/admin/users', async (request, reply) => {
  const users = await getAllUsers();
  return reply.send(users);
});

async function getAllUsers() {
  return [{ id: 1, email: 'admin@example.com' }];
}

export default app;
