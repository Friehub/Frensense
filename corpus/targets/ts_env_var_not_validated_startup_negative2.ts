// SAFE: Uses Zod to validate env vars with type coercion and default values

import { z } from 'zod';

const EnvSchema = z.object({
  DATABASE_URL: z.string().url(),
  API_KEY: z.string().min(1),
  PORT: z.coerce.number().int().positive().default(3000)
});

export function getConfig() {
  const parsed = EnvSchema.safeParse(process.env);
  if (!parsed.success) {
    const missing = parsed.error.issues.map(i => i.path.join('.')).join(', ');
    throw new Error(`Invalid environment configuration: ${missing}`);
  }
  return parsed.data;
}
