// SAFE: Token validation checks the session's revoked_at timestamp against the request time; tokens are invalidated reactively by comparing a cached blacklist
import express from "express";

const revokedTokens = new Set<string>();

export async function revokeSession(req: express.Request, res: express.Response) {
    const tokenId = req.params.tokenId;
    await db.query("UPDATE sessions SET revoked_at = NOW() WHERE id = ?", [tokenId]);
    const token = req.headers.authorization?.split(" ")[1];
    if (token) revokedTokens.add(token);
    res.json({ revoked: true });
}

export async function performSensitiveAction(req: express.Request, res: express.Response) {
    const token = req.headers.authorization?.split(" ")[1];
    if (!token || revokedTokens.has(token)) {
        return res.status(401).json({ error: "Invalid session" });
    }
    const session = await db.query("SELECT * FROM sessions WHERE token = ? AND revoked_at IS NULL", [token]);
    if (session.length === 0) {
        revokedTokens.add(token);
        return res.status(401).json({ error: "Invalid session" });
    }
    await chargeCustomer(req.body.amount);
    res.json({ success: true });
}
