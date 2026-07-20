// SAFE: Job data is validated against a schema before use, and queries use an ORM

import { Worker, Job } from 'bullmq';
import { z } from 'zod';
import { prisma } from './prisma';

const orderSchema = z.object({
  userId: z.string().uuid(),
  productId: z.string().uuid(),
});

const worker = new Worker('order-process', async (job: Job) => {
  const parsed = orderSchema.parse(job.data);

  const user = await prisma.user.findUnique({ where: { id: parsed.userId } });
  const product = await prisma.product.findUnique({ where: { id: parsed.productId } });

  await prisma.order.create({
    data: { userId: parsed.userId, productId: parsed.productId },
  });
}, { connection: { host: 'localhost' } });
