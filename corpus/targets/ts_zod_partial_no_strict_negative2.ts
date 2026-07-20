// SAFE: uses explicit .pick() to define exactly which fields can be updated

import { z } from 'zod';

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
  role: z.enum(['user', 'admin']),
  isActive: z.boolean(),
});

const UserUpdateSchema = UserSchema.pick({
  name: true,
  email: true,
}).partial();

function updateUser(id: string, data: unknown) {
  const update = UserUpdateSchema.parse(data);
  return db.user.update({ where: { id }, data: update });
}

const db = { user: { update: (args: any) => args } };
