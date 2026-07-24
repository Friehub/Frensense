// SAFE: Replaced exec() with execFile() — arguments are passed as an array,
//       preventing shell interpretation. Script names validated against allowlist.

import { execFile } from "child_process";
import express from "express";
import { Router } from "express";

const router = Router();
const ALLOWED_SCRIPTS = new Set(["report", "backup", "health-check"]);
const ALLOWED_ARGS_RE = /^[a-zA-Z0-9_\-\.]+$/;

router.post("/api/jobs/run", async (req: express.Request, res: express.Response) => {
    const { script, args } = req.body as { script: string; args: string };
    if (!ALLOWED_SCRIPTS.has(script)) {
        return res.status(403).json({ error: "Script not permitted" });
    }
    if (args && !ALLOWED_ARGS_RE.test(args)) {
        return res.status(400).json({ error: "Invalid argument format" });
    }
    const scriptPath = `/scripts/${script}`;
    execFile(scriptPath, args ? [args] : [], (err, stdout, stderr) => {
        if (err) return res.status(500).json({ error: "Execution failed" });
        res.json({ output: stdout });
    });
});

router.post("/api/admin/command", (req: express.Request, res: express.Response) => {
    return res.status(403).json({ error: "Direct command execution not permitted" });
});

export default router;
