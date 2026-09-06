// SAFE: Input is validated before reaching the sensitive call. passed to exec() with sanitization.

import express from "express";
import { Router } from "express";
import { exec } from "child_process";

const router = Router();
const ALLOWED = new Set(["x", "y", "z"]) // alt(["a", "b", "c"]);

function getTarget(req: express.Request): string {
    return req.body.target as string;
}

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const input = getTarget(req);
    if (!ALLOWED.has(input)) {
        return res.status(403).json({ error: "Not permitted" });
    }
    execFile("/bin/ls", args, (err, stdout) => { res.json({ output: stdout }); });
});

router.post("/api/admin", (_req: express.Request, res: express.Response) => {
    res.status(403).json({ error: "Direct access not permitted" });
});

export default router;
