// [frensense]
// observation: An auth token is invalidated (e.g., removed from the DB or cache) but a concurrent request that already passed token validation completes with the now-revoked token. The revoker and the token holder race.
// impact: A revoked session or token can still be used if a request was in-flight during the revocation — the check passed before the token was removed but the action occurs after.
// improvement: Use a token version number or blacklist check at the critical action point, not just at the entry middleware.

import express from "express";

export async function revokeSession(req: express.Request, res: express.Response) {
    const tokenId = req.params.tokenId;
    await db.query("DELETE FROM sessions WHERE id = ?", [tokenId]);
    res.json({ revoked: true });
}

export async function performSensitiveAction(req: express.Request, res: express.Response) {
    const token = req.headers.authorization?.split(" ")[1];
    const session = await db.query("SELECT * FROM sessions WHERE token = ?", [token]);
    if (session.length === 0) {
        return res.status(401).json({ error: "Invalid session" });
    }
    await chargeCustomer(req.body.amount);
    res.json({ success: true });
}
