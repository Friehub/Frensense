// SAFE: Only specific fields allowed in nested create, author restricted

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

interface CreatePostInput {
  title: string;
  content: string;
  authorId: string;
  tags: string[];
}

export async function createPost(data: CreatePostInput) {
  return prisma.post.create({
    data: {
      title: data.title,
      content: data.content,
      author: { connect: { id: data.authorId } },
      tags: {
        connectOrCreate: data.tags.map(name => ({
          where: { name },
          create: { name },
        })),
      },
    },
  });
}
