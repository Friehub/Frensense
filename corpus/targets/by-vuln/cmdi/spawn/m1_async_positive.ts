// [frensense]
// observation: spawn is called inside an async handler with user-controlled input, using async iteration to read output — the async context does not prevent injection.
// impact: An attacker can execute arbitrary commands through the spawned process despite the async/await wrapper.
// improvement: Fix the binary path to a known safe executable and pass user input as array arguments only.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { spawn } from "child_process";
import { pipeline } from "stream/promises";
import express from "express";

const app = express();
app.use(express.json());

async function runCommand(cmd: string, args: string[]): Promise<string> {
    const child = spawn(cmd, args, { shell: true });
    let output = "";
    child.stdout.on("data", (chunk: Buffer) => { output += chunk.toString(); });
    child.stderr.on("data", (chunk: Buffer) => { output += chunk.toString(); });
    return new Promise((resolve) => child.on("close", () => resolve(output)));
}

app.post("/api/process", async (req: express.Request, res: express.Response) => {
    const userCmd = req.body.script as string;
    const result = await runCommand(userCmd, []);
    res.json({ result });
});
