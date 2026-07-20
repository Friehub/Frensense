// [frensense]
// observation: Zod's `.catch()` is used with a default value that is unsafe, such as `null`, `undefined`, or `-1`, masking validation errors.
// impact: Invalid input silently returns the fallback value instead of throwing, which can lead to null pointer exceptions, negative IDs, or other logic errors downstream.
// improvement: Avoid `.catch()` with unsafe defaults; use `.safeParse()` and handle validation errors explicitly.

import { z } from 'zod';

const UserIdSchema = z.string().uuid().catch('fallback');

function lookupUser(data: unknown) {
  const userId = UserIdSchema.parse(data);
  return db.user.findUnique({ where: { id: userId } });
}
