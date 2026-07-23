// SAFE: Uses a token version number that is incremented on revocation; the version is checked at the start of the sensitive action, not just at middleware time
import express from "express";

export async function revokeSession(req: express.Request, res: express.Response) {
    const tokenId = req.params.tokenId;
    await db.query("UPDATE sessions SET token_version = token_version + 1, revoked_at = NOW() WHERE id = ?", [tokenId]);
    res.json({ revoked: true });
}

export async function performSensitiveAction(req: express.Request, res: express.Response) {
    const token = req.headers.authorization?.split(" ")[1];
    const session = await db.query(
        "SELECT * FROM sessions WHERE token = ? AND revoked_at IS NULL",
        [token]
    );
    if (session.length === 0) {
        return res.status(401).json({ error: "Invalid session" });
    }
    const dbVersion = session[0].token_version;
    if (session[0].client_token_version !== dbVersion) {
        return res.status(401).json({ error: "Token revoked" });
    }
    await chargeCustomer(req.body.amount);
    res.json({ success: true });
}
