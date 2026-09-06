// [frensense]
// observation: promisify(exec) is used with user-controlled input from req.body.cmd, creating an async wrapper around shell injection.
// impact: An attacker can execute arbitrary commands through the promisified exec call, with the async wrapper providing no security benefit.
// improvement: Use promisify(execFile) instead and pass arguments as an array.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import { promisify } from "util";
import express from "express";

const execAsync = promisify(exec);
const router = express.Router();

router.post("/api/run", async (req: express.Request, res: express.Response) => {
    const command = req.body.cmd as string;
    const { stdout, stderr } = await execAsync(command);
    res.json({ output: stdout, error: stderr });
});

export default router;
