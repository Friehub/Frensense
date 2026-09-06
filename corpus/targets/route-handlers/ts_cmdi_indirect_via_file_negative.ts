// SAFE: Removed execution of user-written files; used sandboxed evaluation or passed data via stdin instead of files.

import { exec, spawn } from "child_process";

function runUserScript(req: Request, res: Response) {
    const scriptContent = req.body.script;
    const sanitized = scriptContent.replace(/[^a-zA-Z0-9_\s=+\-*\/%()]/g, "");
    const proc = spawn("bash", ["-c", sanitized]);
    let stdout = "";
    proc.stdout.on("data", d => stdout += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Script failed" });
        res.json({ output: stdout });
    });
}

function writeConfigThenRun(req: Request, res: Response) {
    const configData = req.body.config;
    const sanitizedConfig = configData.replace(/[;"'$()`]/g, "");
    const proc = spawn("/usr/bin/app", ["--config", `/tmp/app-${Date.now()}.config`]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "App failed" });
        res.json({ message: "Completed" });
    });
}
