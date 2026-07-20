// SAFE: If using `.catch()`, the default is a sentinel value that is handled explicitly downstream

import { z } from 'zod';

const INVALID_USER_ID = 'INVALID_USER' as const;

const UserIdSchema = z.string().uuid().catch(INVALID_USER_ID);

function lookupUser(data: unknown) {
  const userId = UserIdSchema.parse(data);
  if (userId === INVALID_USER_ID) {
    throw new TRPCError({ code: 'BAD_REQUEST', message: 'Invalid user ID' });
  }
  return db.user.findUnique({ where: { id: userId } });
}
