// [frensense]
// observation: getServerSideProps reads a query parameter from context.query and uses it directly in a database query without validation.
// impact: Attackers can inject malicious operators via URL parameters, leading to unauthorized data access or injection.
// improvement: Validate and sanitize query parameters before using them in database queries in getServerSideProps.

import prisma from '@/lib/prisma';
import { GetServerSidePropsContext } from 'next';

export async function getServerSideProps(context: GetServerSidePropsContext) {
  const minPrice = context.query.minPrice as string;
  const products = await prisma.product.findMany({
    where: {
      price: { gte: parseFloat(minPrice) },
    },
  });
  return { props: { products } };
}
