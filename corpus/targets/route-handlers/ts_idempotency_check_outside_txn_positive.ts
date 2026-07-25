// [frensense]
// observation: The idempotency key is checked in the database, and then the operation is performed using that key — but the check and write are not in a single atomic transaction. A concurrent request with the same key can pass the check before either write completes.
// impact: Duplicate processing of the same idempotent operation (e.g., double charge, duplicate order, double webhook processing).
// improvement: Use a single atomic INSERT with a unique constraint on the idempotency key, or wrap the check+write in a database transaction with proper isolation.
// cwe: CWE-754
// cvss: 6.5
// owasp: A04:2021
// severity: Medium

import express from "express";

export async function processPayment(req: express.Request, res: express.Response) {
    const idempotencyKey = req.headers["idempotency-key"] as string;
    const amount = req.body.amount;

    const existing = await db.query("SELECT id FROM processed_payments WHERE idempotency_key = ?", [idempotencyKey]);
    if (existing.length > 0) {
        return res.json({ status: "already_processed", paymentId: existing[0].id });
    }

    const result = await db.query("INSERT INTO processed_payments (amount, idempotency_key) VALUES (?, ?)", [amount, idempotencyKey]);
    await chargeCustomer(amount);
    res.json({ status: "success", paymentId: result.insertId });
}

export async function createOrder(req: express.Request, res: express.Response) {
    const key = req.headers["idempotency-key"] as string;
    const existing = await db.query("SELECT id FROM orders WHERE idempotency_key = ?", [key]);
    if (existing.length > 0) {
        return res.json({ orderId: existing[0].id });
    }
    const order = await db.query("INSERT INTO orders (idempotency_key, status) VALUES (?, 'pending')", [key]);
    await reserveInventory(req.body.items);
    res.json({ orderId: order.insertId });
}
