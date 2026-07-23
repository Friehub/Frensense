// [frensense]
// observation: Two subscription activation requests can run concurrently; both check whether a subscription exists and both proceed to activate a new subscription, creating duplicate subscriptions.
// impact: The same user ends up with two active subscriptions, double-billed for the same period, or duplicate access grants.
// improvement: Use a unique constraint on user_id + plan_id in active subscriptions, or wrap the check+create in a transaction.

import express from "express";

export async function activateSubscription(req: express.Request, res: express.Response) {
    const userId = req.session.userId;
    const planId = req.body.planId;

    const existing = await db.query("SELECT id FROM subscriptions WHERE user_id = ? AND status = 'active'", [userId]);
    if (existing.length > 0) {
        return res.status(400).json({ error: "Already have an active subscription" });
    }

    await db.query("INSERT INTO subscriptions (user_id, plan_id, status, starts_at) VALUES (?, ?, 'active', NOW())", [userId, planId]);
    await chargeUser(userId, planId);
    res.json({ success: true });
}
