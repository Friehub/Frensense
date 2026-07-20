// SAFE: Validates input with safeParse and handles the error case explicitly

import { z } from 'zod';

const UserIdSchema = z.string().uuid();

function lookupUser(data: unknown) {
  const result = UserIdSchema.safeParse(data);
  if (!result.success) {
    throw new Error('Invalid user ID format');
  }
  return db.user.findUnique({ where: { id: result.data } });
}
