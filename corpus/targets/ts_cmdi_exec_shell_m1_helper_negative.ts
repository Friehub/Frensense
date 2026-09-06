// SAFE: Helper uses execFile with array args instead of shell string.

import { execFile } from "child_process";
import express from "express";

const app = express();
const ALLOWED = new Set(["ls", "cat", "echo"]);

function runSafe(cmd: string, args: string[]): Promise<string> {
    return new Promise((resolve, reject) => {
        execFile(cmd, args, (err, stdout) => {
            if (err) reject(err);
            else resolve(stdout);
        });
    });
}

app.post("/run", async (req: express.Request, res: express.Response) => {
    const cmd = req.body.cmd as string;
    if (!ALLOWED.has(cmd)) return res.status(403).json({ error: "Not allowed" });
    const output = await runSafe(cmd, req.body.args ?? []);
    res.json({ output });
});
