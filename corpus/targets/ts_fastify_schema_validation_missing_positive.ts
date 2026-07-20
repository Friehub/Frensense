// [frensense]
// observation: A Fastify route handler reads request body or query parameters without a JSON schema validator, accepting arbitrary unvalidated input.
// impact: Unvalidated input can lead to injection attacks, mass assignment, or type confusion, and bypasses Fastify's automatic input coercion.
// improvement: Define a JSON schema for the route using schema: { body, querystring, params } to validate and coerce all inputs.

import Fastify from 'fastify';

const app = Fastify();

app.post<{ Body: { email: string; role: string } }>('/users', async (request, reply) => {
  const { email, role } = request.body;
  const user = await createUserInDb(email, role);
  return reply.send(user);
});

async function createUserInDb(email: string, role: string) {
  return { id: 1, email, role };
}

export default app;
