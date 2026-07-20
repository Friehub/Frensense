// SAFE: Uses Prisma with UTC-normalized DateTime columns for all booking times

export async function createBookingSlot(prisma: PrismaClient, resourceId: string, startTime: Date, endTime: Date) {
  const slot = await prisma.bookingSlot.create({
    data: {
      resourceId,
      startTimeUtc: startTime,
      endTimeUtc: endTime,
    },
  });

  return { created: true, slotId: slot.id };
}

export async function getAvailableSlots(prisma: PrismaClient, resourceId: string, date: Date, userTimezone: string) {
  const dayStart = new Date(date);
  dayStart.setUTCHours(0, 0, 0, 0);

  const dayEnd = new Date(date);
  dayEnd.setUTCHours(23, 59, 59, 999);

  const slots = await prisma.bookingSlot.findMany({
    where: {
      resourceId,
      startTimeUtc: { gte: dayStart, lte: dayEnd },
    },
    orderBy: { startTimeUtc: 'asc' },
  });

  return slots.map((slot) => ({
    id: slot.id,
    startTimeUtc: slot.startTimeUtc,
    endTimeUtc: slot.endTimeUtc,
    startTimeLocal: slot.startTimeUtc.toLocaleString('en-US', { timeZone: userTimezone }),
    endTimeLocal: slot.endTimeUtc.toLocaleString('en-US', { timeZone: userTimezone }),
  }));
}
