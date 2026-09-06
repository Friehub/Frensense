// SAFE: Binary is fixed, user input is validated before being passed as array arguments.
// spawn is used with shell: false (default) which prevents shell injection.

import { spawn } from "child_process";
import express from "express";

const SANDBOX = "/usr/local/bin/sandbox-exec";
const ARGS_RE = /^[a-zA-Z0-9_\-\.]+$/;

const router = express.Router();

function validateArgs(args: string[]): boolean {
    return args.every((a) => ARGS_RE.test(a));
}

router.post("/api/execute", (req: express.Request, res: express.Response) => {
    const args = req.body.args as string[] || [];
    if (!validateArgs(args)) {
        return res.status(400).json({ error: "Invalid arguments" });
    }
    const child = spawn(SANDBOX, args, { stdio: "pipe" });
    let output = "";
    child.stdout.on("data", (d: Buffer) => { output += d.toString(); });
    child.on("close", (code) => {
        res.json({ exitCode: code, output });
    });
});

export default router;
