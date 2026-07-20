// SAFE: Check and write are wrapped in a Prisma transaction with SERIALIZABLE isolation; the unique constraint provides a second defense layer
import express from "express";

export async function processPayment(req: express.Request, res: express.Response) {
    const idempotencyKey = req.headers["idempotency-key"] as string;
    const amount = req.body.amount;

    const result = await prisma.$transaction(async (tx) => {
        const existing = await tx.processedPayment.findUnique({
            where: { idempotencyKey },
        });
        if (existing) {
            return { status: "already_processed" as const, paymentId: existing.id };
        }
        const payment = await tx.processedPayment.create({
            data: { amount, idempotencyKey },
        });
        await chargeCustomer(tx, amount);
        return { status: "success" as const, paymentId: payment.id };
    }, { isolationLevel: "Serializable" });

    res.json(result);
}
