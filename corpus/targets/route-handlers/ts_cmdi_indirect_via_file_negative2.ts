// SAFE: Used a function queue with fixed scripts instead of writing user input to executable files.

import { spawn } from "child_process";

const FIXED_SCRIPTS: Record<string, string[]> = {
    "backup": ["/usr/local/bin/backup.sh"],
    "cleanup": ["/usr/local/bin/cleanup.sh"],
    "report": ["/usr/local/bin/generate-report.sh"],
};

function runUserScript(req: Request, res: Response) {
    const scriptName = req.body.scriptName;
    const script = FIXED_SCRIPTS[scriptName];
    if (!script) {
        res.status(400).json({ error: "Unknown script" });
        return;
    }
    const proc = spawn("bash", script);
    let stdout = "";
    proc.stdout.on("data", d => stdout += d);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "Script failed" });
        res.json({ output: stdout });
    });
}

function writeConfigThenRun(req: Request, res: Response) {
    const configPath = "/etc/app/default.conf";
    const proc = spawn("/usr/bin/app", ["--config", configPath]);
    proc.on("close", code => {
        if (code !== 0) return res.status(500).json({ error: "App failed" });
        res.json({ message: "Completed" });
    });
}
