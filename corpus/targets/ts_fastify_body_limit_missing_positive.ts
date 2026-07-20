// [frensense]
// observation: A Fastify route does not set a bodyLimit, accepting arbitrarily large request bodies.
// impact: An attacker can send a large payload that exhausts server memory, leading to denial of service.
// improvement: Set a bodyLimit on the route schema or globally in the Fastify constructor to reject oversized payloads.

import Fastify from 'fastify';

const app = Fastify();

app.post<{ Body: { content: string } }>('/upload', async (request, reply) => {
  const { content } = request.body;
  await storeContent(content);
  return reply.send({ ok: true });
});

async function storeContent(content: string) {
  return;
}

export default app;
