// [frensense]
// observation: A Next.js server action accepts user input and uses it directly in operations like database writes without any validation or sanitization.
// impact: Attackers can inject malicious data, causing stored XSS, SQL injection, or data corruption.
// improvement: Validate and sanitize all input in server actions before using it in any operation.

'use server';

import prisma from '@/lib/prisma';

export async function createPost(formData: FormData) {
  const title = formData.get('title') as string;
  const content = formData.get('content') as string;
  await prisma.post.create({
    data: { title, content },
  });
}
