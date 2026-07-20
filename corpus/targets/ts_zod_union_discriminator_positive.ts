// [frensense]
// observation: A Zod union is used without a `.discriminatedUnion()`, relying on ordered `.or()` matching that can misidentify input shapes.
// impact: When input ambiguously matches multiple union members, Zod picks the first match, potentially interpreting the data as the wrong type and leading to incorrect business logic.
// improvement: Use `.discriminatedUnion('type', [...])` with a common discriminator field to disambiguate union members deterministically.

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

const EventSchema = z.union([CreateEventSchema, UpdateEventSchema]);

function handleEvent(data: unknown) {
  const event = EventSchema.parse(data);
  if (event.kind === 'create') {
    return db.event.create({ data: { title: event.title } });
  }
  return db.event.update({ where: { id: event.id }, data: { title: event.title } });
}
