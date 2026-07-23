// SAFE: Input is validated manually with a library before use in the handler

import Fastify from 'fastify';
import { z } from 'zod';

const app = Fastify();

const createUserSchema = z.object({
  email: z.string().email(),
  role: z.enum(['admin', 'user', 'viewer']),
});

app.post('/users', async (request, reply) => {
  const result = createUserSchema.safeParse(request.body);
  if (!result.success) {
    return reply.status(400).send({ error: 'Validation failed', details: result.error.flatten() });
  }
  const { email, role } = result.data;
  const user = await createUserInDb(email, role);
  return reply.send(user);
});

async function createUserInDb(email: string, role: string) {
  return { id: 1, email, role };
}

export default app;
