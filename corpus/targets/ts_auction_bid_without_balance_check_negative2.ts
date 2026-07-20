// SAFE: Uses Prisma with balance check, fund hold, and previous bidder release in a transaction

export async function placeBid(prisma: PrismaClient, userId: string, auctionId: string, amount: number) {
  return prisma.$transaction(async (tx) => {
    const auction = await tx.auction.findUnique({ where: { id: auctionId } });
    if (!auction || auction.endTime < new Date() || auction.status !== 'ACTIVE') {
      throw new Error('Auction not found or ended');
    }

    if (amount <= auction.currentBid) {
      throw new Error('Bid must be higher than current bid');
    }

    const user = await tx.user.findUnique({ where: { id: userId } });
    if (!user || Number(user.balance) < amount) {
      throw new Error('Insufficient balance');
    }

    // Hold the new bidder's funds
    await tx.user.update({
      where: { id: userId },
      data: { balance: { decrement: amount }, heldBalance: { increment: amount } },
    });

    // Release the previous bidder's funds
    if (auction.bidderId) {
      await tx.user.update({
        where: { id: auction.bidderId },
        data: { balance: { increment: auction.currentBid }, heldBalance: { decrement: auction.currentBid } },
      });
    }

    await tx.bid.create({
      data: { userId, auctionId, amount },
    });

    await tx.auction.update({
      where: { id: auctionId },
      data: { currentBid: amount, bidderId: userId },
    });

    return { bidPlaced: true };
  });
}
