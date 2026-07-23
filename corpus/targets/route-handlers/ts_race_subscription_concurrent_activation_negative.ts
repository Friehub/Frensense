// SAFE: Uses a unique constraint on user_id + status to prevent duplicate active subscriptions atomically
import express from "express";

export async function activateSubscription(req: express.Request, res: express.Response) {
    const userId = req.session.userId;
    const planId = req.body.planId;

    try {
        await db.query(
            "INSERT INTO subscriptions (user_id, plan_id, status, starts_at) VALUES (?, ?, 'active', NOW())",
            [userId, planId]
        );
    } catch (err: any) {
        if (err.code === "ER_DUP_ENTRY" || err.code === "SQLITE_CONSTRAINT") {
            return res.status(400).json({ error: "Already have an active subscription" });
        }
        throw err;
    }
    await chargeUser(userId, planId);
    res.json({ success: true });
}
