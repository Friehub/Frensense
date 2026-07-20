// SAFE: Uses `.or()` with mutually exclusive schemas that cannot overlap, guaranteeing correct matching

import { z } from 'zod';

const CreateEventSchema = z.object({
  kind: z.literal('create'),
  title: z.string(),
  id: z.never().optional(),
});

const UpdateEventSchema = z.object({
  kind: z.literal('update'),
  id: z.string(),
  title: z.string().optional(),
});

const EventSchema = z.discriminatedUnion('kind', [CreateEventSchema, UpdateEventSchema]);

function handleEvent(data: unknown) {
  const event = EventSchema.parse(data);
  switch (event.kind) {
    case 'create':
      return db.event.create({ data: { title: event.title } });
    case 'update':
      return db.event.update({ where: { id: event.id }, data: { title: event.title } });
  }
}
