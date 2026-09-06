// [frensense]
// observation: User input flows through a promise chain before reaching exec(), obfuscating the taint path but providing no security.
// impact: The promise wrapper does not sanitize the input — same exploitation as direct execSync injection.
// improvement: Sanitize input at the earliest point. Use execFile with array arguments after validation.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { execSync } from "child_process";
import express from "express";

const router = express.Router();

function sanitizeName(input: string): string {
    return input.replace(/\s+/g, "_");
}

router.post("/api/task", (req: express.Request, res: express.Response) => {
    const raw = req.body.taskName as string;
    const cleaned = sanitizeName(raw);
    const command = `echo ${cleaned}`;
    try {
        const out = execSync(command, { encoding: "utf8" });
        res.json({ result: out.trim() });
    } catch {
        res.sendStatus(500);
    }
});

export default router;
