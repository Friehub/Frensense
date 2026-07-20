// SAFE: Uses `.discriminatedUnion()` with the `kind` field for deterministic disambiguation

import { z } from 'zod';

const CreateEventSchema = z.object({
  kind: z.literal('create'),
  title: z.string(),
});

const UpdateEventSchema = z.object({
  kind: z.literal('update'),
  id: z.string(),
  title: z.string().optional(),
});

const EventSchema = z.discriminatedUnion('kind', [CreateEventSchema, UpdateEventSchema]);

function handleEvent(data: unknown) {
  const event = EventSchema.parse(data);
  if (event.kind === 'create') {
    return db.event.create({ data: { title: event.title } });
  }
  return db.event.update({ where: { id: event.id }, data: { title: event.title } });
}
