// SAFE: Input is validated with zod before being used in database operations

'use server';

import prisma from '@/lib/prisma';
import { z } from 'zod';

const postSchema = z.object({
  title: z.string().min(1).max(200),
  content: z.string().min(1).max(10000),
});

export async function createPost(formData: FormData) {
  const raw = {
    title: formData.get('title'),
    content: formData.get('content'),
  };
  const parsed = postSchema.safeParse(raw);
  if (!parsed.success) throw new Error('Invalid input');
  await prisma.post.create({
    data: parsed.data,
  });
}
