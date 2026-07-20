// SAFE: Only non-sensitive fields are returned in props

import prisma from '@/lib/prisma';

export async function getServerSideProps() {
  const user = await prisma.user.findUnique({
    where: { id: '1' },
    select: { id: true, name: true, avatar: true }
  });
  return {
    props: { user }
  };
}
