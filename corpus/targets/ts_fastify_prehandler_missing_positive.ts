// [frensense]
// observation: A Fastify route does not register a preHandler hook for authentication, leaving the endpoint accessible without authorization.
// impact: Unauthenticated users can call the endpoint, potentially accessing or modifying sensitive data.
// improvement: Register a preHandler authentication hook on the route or use app.addHook('preHandler', authHook) globally.

import Fastify from 'fastify';

const app = Fastify();

app.get<{ Headers: { authorization?: string } }>('/admin/users', async (request, reply) => {
  const users = await getAllUsers();
  return reply.send(users);
});

async function getAllUsers() {
  return [{ id: 1, email: 'admin@example.com' }];
}

export default app;
