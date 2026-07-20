// SAFE: Data is explicitly serialized to strip sensitive fields before sending to the client

import prisma from '@/lib/prisma';

function sanitizeUser(user: any) {
  return {
    id: user.id,
    name: user.name,
    avatar: user.avatar
  };
}

export async function getServerSideProps() {
  const user = await prisma.user.findUnique({ where: { id: '1' } });
  if (!user) return { notFound: true };
  return {
    props: { user: sanitizeUser(user) }
  };
}
