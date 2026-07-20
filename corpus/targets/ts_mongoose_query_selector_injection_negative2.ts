// SAFE: Uses Zod to validate and transform the query filter before passing it to Mongoose

import mongoose from 'mongoose';
import { z } from 'zod';

const User = mongoose.model('User', new mongoose.Schema({
  name: String,
  email: String,
  role: String
}));

const UserFilterSchema = z.object({
  name: z.string().optional(),
  email: z.string().email().optional(),
  role: z.enum(['admin', 'user', 'moderator']).optional()
});

export async function findUsers(body: any) {
  const parsed = UserFilterSchema.safeParse(body);
  if (!parsed.success) throw new Error('Invalid filter');
  return User.find(parsed.data);
}
