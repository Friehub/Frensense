// SAFE: a typed API client wraps fetch and validates the response shape

import { z } from 'zod';

function createApiClient(baseUrl: string) {
  async function get<T>(path: string, schema: z.ZodType<T>): Promise<T> {
    const response = await fetch(`${baseUrl}${path}`);
    const raw = await response.json();
    return schema.parse(raw);
  }
  return { get };
}

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
});

async function getUser(userId: string) {
  const api = createApiClient('/api');
  const data = await api.get(`/users/${userId}`, UserSchema);
  return { id: data.id, name: data.name, email: data.email };
}
