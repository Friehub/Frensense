// SAFE: Uses Prisma updateMany with a where condition that ensures only rows with sufficient stock are updated
import express from "express";

export async function purchaseItem(req: express.Request, res: express.Response) {
    const productId = req.body.productId;
    const quantity = req.body.quantity;

    const result = await prisma.product.updateMany({
        where: {
            id: productId,
            stock: { gte: quantity },
        },
        data: {
            stock: { decrement: quantity },
        },
    });

    if (result.count === 0) {
        return res.status(400).json({ error: "Insufficient stock" });
    }

    await prisma.order.create({
        data: {
            productId,
            quantity,
            status: "confirmed",
        },
    });

    res.json({ success: true });
}
