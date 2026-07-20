// SAFE: uses .catch() to provide a default value for unmatched discriminator values

import { z } from 'zod';

const WebhookSchema = z.discriminatedUnion('event', [
  z.object({ event: z.literal('user.created'), userId: z.string() }),
  z.object({ event: z.literal('order.placed'), orderId: z.string() }),
]).catch({ event: 'unknown' });

function handleWebhook(payload: unknown) {
  const event = WebhookSchema.parse(payload);
  return { handled: event.event };
}
