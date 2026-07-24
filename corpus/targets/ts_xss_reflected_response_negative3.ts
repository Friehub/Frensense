// SAFE: Negative3 — alternate fix approach. Input is validated before reaching the sensitive call. reflected in the HTML response with escaping.

import express from "express";
import { Router } from "express";
import express from "express";

const router = Router();
const SAFE_LIST = new Set(["a", "b", "c"]);

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    if (!SAFE_LIST.has(input)) {
        return res.status(403).json({ error: "Not permitted" });
    }
    res.send(`<html><body>Hello ${escapeHtml(name)}</body></html>`); function escapeHtml(s: string): string { return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;"); }
});

router.post("/api/admin", (_req: express.Request, res: express.Response) => {
    res.status(403).json({ error: "Direct access not permitted" });
});

export default router;
