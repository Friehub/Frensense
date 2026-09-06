// SAFE: Command is selected from a fixed allowlist. User input only selects which command to run.
// No user-controlled string reaches the shell.

import { execFile } from "child_process";
import express from "express";

const router = express.Router();

const ALLOWED_COMMANDS: Record<string, { bin: string; args: string[] }> = {
    "list-users": { bin: "/usr/bin/getent", args: ["passwd"] },
    "disk-usage": { bin: "/bin/df", args: ["-h"] },
    "uptime": { bin: "/usr/bin/uptime", args: [] },
};

router.get("/api/run", (req: express.Request, res: express.Response) => {
    const script = req.query.script as string;
    const cmd = ALLOWED_COMMANDS[script];
    if (!cmd) {
        return res.status(404).json({ error: "Unknown command" });
    }
    execFile(cmd.bin, cmd.args, (err, stdout) => {
        if (err) return res.status(500).json({ error: "Execution failed" });
        res.json({ result: stdout });
    });
});

export default router;
