// SAFE: Replaced exec() with execFile(). Arguments are passed as an array, preventing shell injection.
// The binary path is fixed and user input is validated against a strict pattern.

import { execFile } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

const ALLOWED_ARGS_RE = /^[a-zA-Z0-9\-_]+$/;
const SANDBOX_BIN = "/usr/local/bin/sandbox-run";

function validateInput(input: string): string | null {
    if (!ALLOWED_ARGS_RE.test(input)) {
        return null;
    }
    return input;
}

app.post("/api/deploy", (req: express.Request, res: express.Response) => {
    const userInput = req.body.cmd as string;
    const safe = validateInput(userInput);
    if (!safe) {
        return res.status(400).json({ error: "Invalid input" });
    }
    execFile(SANDBOX_BIN, [safe], (error, stdout, stderr) => {
        if (error) {
            return res.status(500).json({ error: stderr });
        }
        res.json({ output: stdout });
    });
});
