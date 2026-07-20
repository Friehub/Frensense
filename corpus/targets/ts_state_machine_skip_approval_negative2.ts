// SAFE: Uses a state machine class with explicit transition guards for multi-step workflows

class ContentStateMachine {
  private allowedTransitions: Record<string, string[]> = {
    DRAFT: ['REVIEW'],
    REVIEW: ['APPROVED', 'REJECTED'],
    APPROVED: ['SHIPPED'],
    SHIPPED: [],
    REJECTED: ['DRAFT'],
  };

  canTransition(current: string, next: string): boolean {
    return this.allowedTransitions[current]?.includes(next) ?? false;
  }
}

export async function shipContent(prisma: PrismaClient, contentId: string) {
  const content = await prisma.content.findUnique({
    where: { id: contentId },
    select: { status: true },
  });

  if (!content) throw new Error('Not found');

  const sm = new ContentStateMachine();
  if (!sm.canTransition(content.status, 'SHIPPED')) {
    throw new Error(
      `Cannot ship from ${content.status}; must be APPROVED`
    );
  }

  const updated = await prisma.content.updateMany({
    where: { id: contentId, status: content.status },
    data: { status: 'SHIPPED' },
  });

  if (updated.count === 0) {
    throw new Error('Concurrent modification detected');
  }

  await notifySubscribers(contentId, prisma);
}
