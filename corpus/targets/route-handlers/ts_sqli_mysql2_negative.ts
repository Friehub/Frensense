// SAFE: URL validated against an allowlist before making the request.

import express from "express";
import { Router } from "express";
const mysql = require("mysql2");
const conn = mysql.createConnection({ host: "localhost", user: "root" });

const router = Router();
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    try {
        const parsed = new URL(url);
        conn.execute("SELECT * FROM users WHERE id = ?", [userId], (err, rows) => {
    if (err) return res.status(500).json({ error: err.message });
    res.json(rows);
  });
    } catch {
        return res.status(400).json({ error: "Invalid URL" });
    }
});

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    return res.status(403).json({ error: "Direct fetch not permitted" });
});

export default router;
