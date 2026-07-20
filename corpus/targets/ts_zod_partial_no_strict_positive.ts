// [frensense]
// observation: Zod's .partial() is used without chaining .strict(), allowing arbitrary extra fields to pass through validation.
// impact: In API patch or update endpoints, extra fields from the client are silently accepted and may be persisted to the database, enabling mass assignment attacks or data corruption.
// improvement: Chain .strict() after .partial() to reject unknown fields, or use explicit .pick() / .omit() to define exactly which fields are updatable.

import { z } from 'zod';

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
  role: z.enum(['user', 'admin']),
  isActive: z.boolean(),
});

const UserUpdateSchema = UserSchema.partial();

function updateUser(id: string, data: unknown) {
  const update = UserUpdateSchema.parse(data);
  return db.user.update({ where: { id }, data: update });
}

const db = { user: { update: (args: any) => args } };
