// SAFE: API response is validated against a Zod schema at runtime

import { z } from 'zod';

const UserResponseSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
});

async function getUser(userId: string) {
  const response = await fetch(`/api/users/${userId}`);
  const raw = await response.json();
  const data = UserResponseSchema.parse(raw);
  return { id: data.id, name: data.name, email: data.email };
}
