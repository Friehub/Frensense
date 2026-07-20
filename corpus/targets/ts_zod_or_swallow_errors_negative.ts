// SAFE: Uses a strict enum and handles invalid input explicitly instead of swallowing with a fallback

import { z } from 'zod';

const StatusSchema = z.enum(['active', 'inactive', 'pending']);

function processStatus(data: unknown) {
  const result = StatusSchema.safeParse(data);
  if (!result.success) {
    throw new Error(`Invalid status. Must be one of: active, inactive, pending`);
  }
  return result.data;
}
