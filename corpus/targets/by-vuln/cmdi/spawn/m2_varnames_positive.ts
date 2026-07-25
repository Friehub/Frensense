// [frensense]
// observation: req.query.tool is passed through multiple intermediate variables before reaching spawn() — the indirection does not sanitize the input.
// impact: The extra variable assignments only obscure the taint flow; the spawn call is still vulnerable to command injection.
// improvement: Validate the input at the earliest point. Use a mapping of allowed tools to fixed binary paths.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { spawn } from "child_process";
import express from "express";

const router = express.Router();

router.get("/api/tools", (req: express.Request, res: express.Response) => {
    const selected = req.query.tool as string;
    const tool = selected;
    const path = `/usr/bin/${tool}`;
    const proc = spawn(path, [], { stdio: "pipe" });
    proc.stdout.pipe(res);
});

export default router;
