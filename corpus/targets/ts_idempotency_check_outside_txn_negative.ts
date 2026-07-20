// SAFE: Uses a single atomic INSERT with a UNIQUE constraint on idempotency_key; if the key already exists, the INSERT fails atomically
import express from "express";

export async function processPayment(req: express.Request, res: express.Response) {
    const idempotencyKey = req.headers["idempotency-key"] as string;
    const amount = req.body.amount;

    try {
        const result = await db.query(
            "INSERT INTO processed_payments (amount, idempotency_key) VALUES (?, ?)",
            [amount, idempotencyKey]
        );
        await chargeCustomer(amount);
        res.json({ status: "success", paymentId: result.insertId });
    } catch (err: any) {
        if (err.code === "ER_DUP_ENTRY" || err.code === "SQLITE_CONSTRAINT") {
            const existing = await db.query("SELECT id FROM processed_payments WHERE idempotency_key = ?", [idempotencyKey]);
            return res.json({ status: "already_processed", paymentId: existing[0].id });
        }
        throw err;
    }
}

export async function createOrder(req: express.Request, res: express.Response) {
    const key = req.headers["idempotency-key"] as string;
    try {
        const order = await db.query(
            "INSERT INTO orders (idempotency_key, status) VALUES (?, 'pending')",
            [key]
        );
        await reserveInventory(req.body.items);
        res.json({ orderId: order.insertId });
    } catch (err: any) {
        if (err.code === "ER_DUP_ENTRY" || err.code === "SQLITE_CONSTRAINT") {
            const existing = await db.query("SELECT id FROM orders WHERE idempotency_key = ?", [key]);
            return res.json({ orderId: existing[0].id });
        }
        throw err;
    }
}
