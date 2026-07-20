// [frensense]
// observation: The application checks if a session exists for a user before creating a new one, but the check and creation are not atomic. Two concurrent login requests can both pass the check and create duplicate sessions.
// impact: A user logging in twice rapidly can end up with two valid sessions, each with different session tokens, potentially causing state confusion or security bypass.
// improvement: Use a unique constraint on user_id in the sessions table, or wrap the check+create in a transaction.

import express from "express";

export async function login(req: express.Request, res: express.Response) {
    const { userId } = req.body;
    const existingSession = await db.query("SELECT id FROM sessions WHERE user_id = ? AND expires_at > NOW()", [userId]);
    if (existingSession.length > 0) {
        req.session.sessionId = existingSession[0].id;
        return res.json({ sessionId: existingSession[0].id });
    }
    const session = await db.query("INSERT INTO sessions (user_id, token, expires_at) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 DAY))", [userId, generateToken()]);
    req.session.sessionId = session.insertId;
    res.json({ sessionId: session.insertId });
}
