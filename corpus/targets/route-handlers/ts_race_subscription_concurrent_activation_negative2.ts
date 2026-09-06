// SAFE: Check and create are wrapped in a Prisma transaction with serializable isolation
import express from "express";

export async function activateSubscription(req: express.Request, res: express.Response) {
    const userId = req.session.userId;
    const planId = req.body.planId;

    const result = await prisma.$transaction(async (tx) => {
        const existing = await tx.subscription.findFirst({
            where: { userId, status: "active" },
        });
        if (existing) {
            return { success: false, error: "Already have an active subscription" };
        }
        await tx.subscription.create({
            data: { userId, planId, status: "active", startsAt: new Date() },
        });
        return { success: true };
    }, { isolationLevel: "Serializable" });

    if (result.success) {
        await chargeUser(userId, planId);
        res.json({ success: true });
    } else {
        res.status(400).json({ error: result.error });
    }
}
