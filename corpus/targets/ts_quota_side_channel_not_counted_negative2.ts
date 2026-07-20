// SAFE: Uses Prisma with a unified billable action table

export async function handleToolCall(prisma: PrismaClient, userId: string, toolName: string, args: any) {
  const billableTools = ['llm_chat', 'web_search', 'code_interpreter', 'image_gen'];

  if (billableTools.includes(toolName)) {
    await prisma.$transaction(async (tx) => {
      const quota = await tx.quota.findUnique({ where: { userId } });
      if (!quota || quota.remaining <= 0) {
        throw new Error('Quota exceeded');
      }

      await tx.quota.update({
        where: { userId },
        data: { remaining: { decrement: 1 } },
      });

      await tx.billableAction.create({
        data: { userId, toolName, args: JSON.stringify(args) },
      });
    });
  }

  return env.MCP.invoke(toolName, args);
}
