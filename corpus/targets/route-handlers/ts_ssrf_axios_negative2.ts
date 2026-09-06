// SAFE: URL validated against an allowlist before making the request.
// alt allowlist

import express from "express";
import { Router } from "express";
import axios from "axios";
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
        if (!PERMITTED_HOSTS.has(parsed.hostname)) return res.status(403).json({ error: "Host not allowed" });
    const response = await axios.get(url);
    res.json(response.data);
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
