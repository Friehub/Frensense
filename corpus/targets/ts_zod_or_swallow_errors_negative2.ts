// SAFE: Instead of a permissive `.or()`, the schema maps known values and rejects anything else

import { z } from 'zod';

const StatusSchema = z.union([
  z.enum(['active', 'inactive', 'pending']),
]).refine((val) => ['active', 'inactive', 'pending'].includes(val), {
  message: 'Status must be one of: active, inactive, pending',
});

function processStatus(data: unknown) {
  return StatusSchema.parse(data);
}
