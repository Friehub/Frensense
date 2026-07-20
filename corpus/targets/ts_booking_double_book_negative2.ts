// SAFE: Uses Prisma with a unique constraint on (resourceId, slotStart) and transactional creation

export async function bookSlot(prisma: PrismaClient, userId: string, resourceId: string, slotStart: Date, slotEnd: Date) {
  const existing = await prisma.booking.findFirst({
    where: {
      resourceId,
      slotStart,
      status: 'CONFIRMED',
    },
  });

  if (existing) {
    throw new Error('This time slot is already booked');
  }

  try {
    const booking = await prisma.booking.create({
      data: {
        userId,
        resourceId,
        slotStart,
        slotEnd,
        status: 'CONFIRMED',
      },
    });
    return { booked: true, bookingId: booking.id };
  } catch (e) {
    if ((e as any)?.code === 'P2002') {
      throw new Error('Slot was just taken by another user');
    }
    throw e;
  }
}
