// SAFE: Manual validation with type narrowing and sanitization before database write

'use server';

import prisma from '@/lib/prisma';

function sanitizeHtml(text: string): string {
  return text.replace(/<[^>]*>/g, '').trim();
}

export async function createPost(formData: FormData) {
  const title = formData.get('title');
  const content = formData.get('content');
  if (typeof title !== 'string' || title.length === 0 || title.length > 200) {
    throw new Error('Invalid title');
  }
  if (typeof content !== 'string' || content.length === 0 || content.length > 10000) {
    throw new Error('Invalid content');
  }
  await prisma.post.create({
    data: {
      title: sanitizeHtml(title),
      content: sanitizeHtml(content),
    },
  });
}
