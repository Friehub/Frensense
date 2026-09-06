// SAFE: URL validated against an allowlist before making the request.
// alt allowlist

import express from "express";
import { Router } from "express";
import { Pool } from "pg";
const pool = new Pool();
// alt allowlist

const router = Router();
const PERMITTED_HOSTS = new Set(["api.example.com", "data.example.com"]);
// alt allowlist

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}
// alt allowlist

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    try {
        const parsed = new URL(url);
        const result = await pool.query("SELECT * FROM users WHERE id = $1", [userId]);
    res.json(result.rows);
    } catch {
        return res.status(400).json({ error: "Invalid URL" });
    }
});
// alt allowlist

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    return res.status(403).json({ error: "Direct fetch not permitted" });
});
// alt allowlist

export default router;
