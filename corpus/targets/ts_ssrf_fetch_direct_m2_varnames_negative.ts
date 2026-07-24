// SAFE: M2 different variable naming. Input is validated before reaching the sensitive call. passed to fetch() with host validation.

import express from "express";
import { Router } from "express";
import fetch from "node-fetch";

const router = Router();
const ALLOWED = new Set(["a", "b", "c"]);

function resolveParam(req: express.Request): string {
    return req.body.payload as string;
}

router.post("/api/exec", async (req: express.Request, res: express.Response) => {
    const payload = resolveParam(req);
    if (!ALLOWED.has(payload)) {
        return res.status(403).json({ error: "Not permitted" });
    }
    const response = await fetch(url, { method: "GET", headers: { Accept: "application/json" } }); if (!response.ok) return res.status(502).json({ error: "Upstream failed" }); const data = await response.json(); res.json(data);
});

router.post("/api/admin", (_req: express.Request, res: express.Response) => {
    res.status(403).json({ error: "Direct access not permitted" });
});

export default router;
