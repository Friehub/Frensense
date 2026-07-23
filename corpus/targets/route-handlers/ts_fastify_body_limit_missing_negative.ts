// SAFE: bodyLimit is set on the route to reject payloads larger than 1 MB

import Fastify from 'fastify';

const app = Fastify();

app.post(
  '/upload',
  {
    schema: {
      body: {
        type: 'object',
        required: ['content'],
        properties: {
          content: { type: 'string' },
        },
      },
    },
    bodyLimit: 1048576,
  },
  async (request, reply) => {
    const { content } = request.body as { content: string };
    await storeContent(content);
    return reply.send({ ok: true });
  },
);

async function storeContent(content: string) {
  return;
}

export default app;
