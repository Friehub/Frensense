// [frensense]
// observation: A discriminatedUnion is used without a fallback schema for unknown variants, causing parse failures on unrecognized discriminator values.
// impact: When the input has an unrecognized discriminator value (e.g., a new API version), the parse throws a ZodError that may crash the process or be improperly handled, leaving the system in an inconsistent state.
// improvement: Add a fallback or catch-all schema as the last member of the discriminatedUnion array so unknown variants are handled gracefully.

import { z } from 'zod';

const WebhookSchema = z.discriminatedUnion('event', [
  z.object({ event: z.literal('user.created'), userId: z.string() }),
  z.object({ event: z.literal('order.placed'), orderId: z.string() }),
]);

function handleWebhook(payload: unknown) {
  const event = WebhookSchema.parse(payload);
  return { handled: event.event };
}
