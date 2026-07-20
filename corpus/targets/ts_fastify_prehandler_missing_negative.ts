// SAFE: The route has a preHandler hook that checks authentication before the handler runs

import Fastify from 'fastify';
import type { FastifyRequest, FastifyReply } from 'fastify';

const app = Fastify();

async function authenticate(request: FastifyRequest, reply: FastifyReply) {
  const token = request.headers.authorization;
  if (!token || !token.startsWith('Bearer ')) {
    return reply.status(401).send({ error: 'Unauthorized' });
  }
}

app.get('/admin/users', { preHandler: authenticate }, async (request, reply) => {
  const users = await getAllUsers();
  return reply.send(users);
});

async function getAllUsers() {
  return [{ id: 1, email: 'admin@example.com' }];
}

export default app;
