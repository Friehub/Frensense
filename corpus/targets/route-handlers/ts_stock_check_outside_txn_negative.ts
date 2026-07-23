// SAFE: Stock check and decrement are performed in a single atomic UPDATE with a WHERE condition on the stock level
import express from "express";

export async function purchaseItem(req: express.Request, res: express.Response) {
    const productId = req.body.productId;
    const quantity = req.body.quantity;

    const result = await db.query(
        "UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?",
        [quantity, productId, quantity]
    );

    if (result.affectedRows === 0) {
        return res.status(400).json({ error: "Insufficient stock" });
    }

    await createOrder(productId, quantity);
    res.json({ success: true });
}
