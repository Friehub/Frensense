// SAFE: a catch-all fallback schema is added for unknown event types

import { z } from 'zod';

const WebhookSchema = z.discriminatedUnion('event', [
  z.object({ event: z.literal('user.created'), userId: z.string() }),
  z.object({ event: z.literal('order.placed'), orderId: z.string() }),
  z.object({ event: z.string() }),
]);

function handleWebhook(payload: unknown) {
  const event = WebhookSchema.parse(payload);
  if (event.event === 'user.created') {
    return { handled: 'user' };
  }
  if (event.event === 'order.placed') {
    return { handled: 'order' };
  }
  return { handled: 'unknown' };
}
