// [frensense]
// observation: A nested create accepts user-controlled relation data, allowing the creation of arbitrary related records.
// impact: An attacker can create or connect arbitrary related entities by injecting relation data into the nested create payload.
// improvement: Validate and restrict which relations can be created or connected via nested operations.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createPost(data: {
  title: string;
  content: string;
  author: { connect: { id: string } };
  tags: { create: { name: string }[] };
}) {
  return prisma.post.create({
    data: {
      title: data.title,
      content: data.content,
      author: data.author,
      tags: data.tags,
    },
  });
}
