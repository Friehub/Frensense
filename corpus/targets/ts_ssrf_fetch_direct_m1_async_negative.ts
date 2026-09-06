// SAFE: M1 async variant. Input is validated before reaching the sensitive call. passed to fetch() with host validation.

import express from "express";
import { Router } from "express";
import fetch from "node-fetch";

const router = Router();
const ALLOWED = new Set(["a", "b", "c"]);

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    if (!ALLOWED.has(input)) {
        return res.status(403).json({ error: "Not permitted" });
    }
    const response = await fetch(url, { method: "GET", headers: { Accept: "application/json" } }); if (!response.ok) return res.status(502).json({ error: "Upstream failed" }); const data = await response.json(); res.json(data);
});

router.post("/api/admin", (_req: express.Request, res: express.Response) => {
    res.status(403).json({ error: "Direct access not permitted" });
});

export default router;
