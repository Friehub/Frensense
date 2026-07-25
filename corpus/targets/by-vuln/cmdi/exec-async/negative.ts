// SAFE: execFile replaces execSync. Arguments are passed as an array, preventing shell injection.
// The binary path is fixed, and user input is validated before being passed as an argument.

import { execFile } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

const VALID_CMD_RE = /^[a-z0-9_\-]+$/;

app.post("/api/exec", (req: express.Request, res: express.Response) => {
    const input = req.body.command as string;
    if (!VALID_CMD_RE.test(input)) {
        return res.status(400).json({ error: "Invalid command format" });
    }
    execFile("/usr/local/bin/runner", [input], { encoding: "utf8" }, (err, stdout) => {
        if (err) return res.status(500).json({ error: "Execution failed" });
        res.json({ output: stdout });
    });
});
