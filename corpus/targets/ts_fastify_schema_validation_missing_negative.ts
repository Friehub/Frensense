// SAFE: Route uses a JSON schema to validate and coerce input before the handler runs

import Fastify from 'fastify';

const app = Fastify();

app.post<{ Body: { email: string; role: string } }>(
  '/users',
  {
    schema: {
      body: {
        type: 'object',
        required: ['email', 'role'],
        properties: {
          email: { type: 'string', format: 'email' },
          role: { type: 'string', enum: ['admin', 'user', 'viewer'] },
        },
      },
    },
  },
  async (request, reply) => {
    const { email, role } = request.body;
    const user = await createUserInDb(email, role);
    return reply.send(user);
  },
);

async function createUserInDb(email: string, role: string) {
  return { id: 1, email, role };
}

export default app;
