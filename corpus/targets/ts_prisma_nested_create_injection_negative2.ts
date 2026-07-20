// SAFE: Nested create sanitized by explicitly constructing data

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createPost(data: {
  title: string;
  content: string;
  authorId: string;
  tagIds: string[];
}) {
  return prisma.post.create({
    data: {
      title: data.title,
      content: data.content,
      author: { connect: { id: data.authorId } },
      tags: {
        connect: data.tagIds.map(id => ({ id })),
      },
    },
  });
}
