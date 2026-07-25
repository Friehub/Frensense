// [frensense]
// observation: User-controlled input from req.body.binary is passed to spawn() as the file argument, allowing arbitrary binary execution.
// impact: An attacker can specify any executable path on the system, leading to arbitrary command execution with the process's permissions.
// improvement: Validate the binary path against an allowlist of permitted executables, or use execFile with a fixed binary and array arguments.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { spawn } from "child_process";
import express from "express";

const router = express.Router();

function resolveBinary(req: express.Request): string {
    return req.body.binary as string;
}

router.post("/api/execute", (req: express.Request, res: express.Response) => {
    const bin = resolveBinary(req);
    const args = req.body.args as string[] || [];
    const proc = spawn(bin, args, { stdio: "pipe" });
    let output = "";
    proc.stdout.on("data", (data: Buffer) => { output += data.toString(); });
    proc.stderr.on("data", (data: Buffer) => { output += data.toString(); });
    proc.on("close", (code: number | null) => {
        res.json({ exitCode: code, output });
    });
});

export default router;
