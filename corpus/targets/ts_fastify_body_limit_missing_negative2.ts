// SAFE: Global bodyLimit is set in the Fastify constructor

import Fastify from 'fastify';

const app = Fastify({ bodyLimit: 2097152 });

app.post('/upload', async (request, reply) => {
  const { content } = request.body as { content: string };
  await storeContent(content);
  return reply.send({ ok: true });
});

async function storeContent(content: string) {
  return;
}

export default app;
