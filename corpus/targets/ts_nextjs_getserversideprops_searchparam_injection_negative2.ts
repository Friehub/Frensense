// SAFE: Multiple query parameters validated with zod and safely coerced before query

import prisma from '@/lib/prisma';
import { GetServerSidePropsContext } from 'next';
import { z } from 'zod';

const filterSchema = z.object({
  minPrice: z.coerce.number().min(0).default(0),
  maxPrice: z.coerce.number().min(0).optional(),
  category: z.string().max(50).optional(),
});

export async function getServerSideProps(context: GetServerSidePropsContext) {
  const parsed = filterSchema.safeParse(context.query);
  if (!parsed.success) {
    return { props: { products: [], error: 'Invalid filters' } };
  }
  const where: Record<string, unknown> = {
    price: { gte: parsed.data.minPrice },
  };
  if (parsed.data.maxPrice) (where.price as Record<string, unknown>).lte = parsed.data.maxPrice;
  if (parsed.data.category) where.category = parsed.data.category;
  const products = await prisma.product.findMany({ where });
  return { props: { products } };
}
