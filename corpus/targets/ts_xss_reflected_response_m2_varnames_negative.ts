// SAFE: M2 different variable naming. Input is validated before reaching the sensitive call. reflected in the HTML response with escaping.

import express from "express";
import { Router } from "express";
import express from "express";

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
    res.send(`<html><body>Hello ${escapeHtml(name)}</body></html>`); function escapeHtml(s: string): string { return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;"); }
});

router.post("/api/admin", (_req: express.Request, res: express.Response) => {
    res.status(403).json({ error: "Direct access not permitted" });
});

export default router;
