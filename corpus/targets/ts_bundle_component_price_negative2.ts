// SAFE: Uses Prisma with a dedicated BundleItem model that enforces fixed pricing

export async function addBundleToCart(prisma: PrismaClient, userId: string, bundleId: string) {
  const bundle = await prisma.bundle.findUnique({
    where: { id: bundleId },
    include: { components: true },
  });

  if (!bundle || !bundle.active) throw new Error('Bundle not found');

  await prisma.cartItem.create({
    data: {
      userId,
      bundleId: bundle.id,
      quantity: 1,
      price: bundle.bundlePrice,
      isBundle: true,
    },
  });

  return { added: true };
}
