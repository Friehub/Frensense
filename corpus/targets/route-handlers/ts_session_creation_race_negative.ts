// SAFE: Uses a UNIQUE constraint on user_id in the sessions table; INSERT fails if session exists, so the operation is atomic
import express from "express";

export async function login(req: express.Request, res: express.Response) {
    const { userId } = req.body;
    try {
        const session = await db.query(
            "INSERT INTO sessions (user_id, token, expires_at) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 DAY))",
            [userId, generateToken()]
        );
        req.session.sessionId = session.insertId;
        res.json({ sessionId: session.insertId });
    } catch (err: any) {
        if (err.code === "ER_DUP_ENTRY" || err.code === "SQLITE_CONSTRAINT_UNIQUE") {
            const existing = await db.query("SELECT id FROM sessions WHERE user_id = ? AND expires_at > NOW()", [userId]);
            req.session.sessionId = existing[0].id;
            return res.json({ sessionId: existing[0].id });
        }
        throw err;
    }
}
