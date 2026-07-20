// SAFE: .strict() rejects unknown fields, preventing mass assignment

import { z } from 'zod';

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
  role: z.enum(['user', 'admin']),
  isActive: z.boolean(),
});

const UserUpdateSchema = UserSchema.partial().strict();

function updateUser(id: string, data: unknown) {
  const update = UserUpdateSchema.parse(data);
  return db.user.update({ where: { id }, data: update });
}

const db = { user: { update: (args: any) => args } };
