// [frensense]
// observation: The inventory stock level is checked in application memory, then the stock is decremented — but these operations are not atomic. Concurrent requests can both see sufficient stock and both proceed to oversell.
// impact: Over-selling of inventory — two or more customers can purchase the last item, resulting in negative stock and fulfillment failures.
// improvement: Use an atomic SQL UPDATE with a WHERE clause on the stock level (UPDATE SET stock = stock - qty WHERE stock >= qty).

import express from "express";

export async function purchaseItem(req: express.Request, res: express.Response) {
    const productId = req.body.productId;
    const quantity = req.body.quantity;

    const product = await db.query("SELECT stock FROM products WHERE id = ?", [productId]);
    if (product.length === 0) return res.status(404).json({ error: "Not found" });

    if (product[0].stock < quantity) {
        return res.status(400).json({ error: "Insufficient stock" });
    }

    await db.query("UPDATE products SET stock = stock - ? WHERE id = ?", [quantity, productId]);
    await createOrder(productId, quantity);
    res.json({ success: true });
}
