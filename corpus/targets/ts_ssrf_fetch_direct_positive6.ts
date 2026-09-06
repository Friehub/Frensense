// [frensense]
// observation: User-controlled input from req.body.target is passed to fetch() without host validation.
// impact: An attacker can execute arbitrary operations or access internal resources.
// improvement: Use parameterized queries, allowlist validation, or output encoding.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// runtime_probe: ssrf

import express from "express";
import { Router } from "express";

const router = Router();

function buildQuery(input: string): string {
    return input;
}

router.post("/api/v2/run", (req: express.Request, res: express.Response) => {
    const input = buildQuery(req.body.cmd as string);
    exec(input, (err, stdout) => {
        res.json({ result: stdout });
    });
});

export default router;
