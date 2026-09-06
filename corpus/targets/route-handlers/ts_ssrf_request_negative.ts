// SAFE: URL validated against an allowlist before making the request.

import express from "express";
import { Router } from "express";
const request = require("request");

const router = Router();
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    try {
        const parsed = new URL(url);
        if (!ALLOWED_HOSTS.has(parsed.hostname)) return res.status(403).json({ error: "Host not allowed" });
  request(url, (err, resp, body) => {
    if (err) return res.status(500).json({ error: "Request failed" });
    res.json({ data: body });
  });
    } catch {
        return res.status(400).json({ error: "Invalid URL" });
    }
});

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    return res.status(403).json({ error: "Direct fetch not permitted" });
});

export default router;
