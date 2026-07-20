// SAFE: Query parameter is validated and coerced to expected type before use

import prisma from '@/lib/prisma';
import { GetServerSidePropsContext } from 'next';

export async function getServerSideProps(context: GetServerSidePropsContext) {
  const raw = context.query.minPrice;
  const minPrice = typeof raw === 'string' ? parseFloat(raw) : NaN;
  if (isNaN(minPrice) || minPrice < 0) {
    return { props: { products: [] } };
  }
  const products = await prisma.product.findMany({
    where: {
      price: { gte: minPrice },
    },
  });
  return { props: { products } };
}
