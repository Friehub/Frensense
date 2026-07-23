// SAFE: Uses a Prisma transaction with serializable isolation; the read and write are in the same transaction boundary
import express from "express";

export async function login(req: express.Request, res: express.Response) {
    const { userId } = req.body;

    const result = await prisma.$transaction(async (tx) => {
        const existing = await tx.session.findFirst({
            where: { userId, expiresAt: { gt: new Date() } },
        });
        if (existing) {
            return { sessionId: existing.id, created: false };
        }
        const session = await tx.session.create({
            data: {
                userId,
                token: generateToken(),
                expiresAt: new Date(Date.now() + 86400000),
            },
        });
        return { sessionId: session.id, created: true };
    }, { isolationLevel: "Serializable" });

    req.session.sessionId = result.sessionId;
    res.json(result);
}
