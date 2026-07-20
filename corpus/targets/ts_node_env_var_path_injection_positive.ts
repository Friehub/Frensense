// [frensense]
// observation: The process.env.PATH is modified using user-controlled input without validation, allowing an attacker to prepend a malicious directory.
// impact: An attacker can control which executables are resolved, causing the application to execute a malicious binary instead of the intended one.
// improvement: Validate or sanitize user input before modifying PATH, or use absolute paths for all spawned executables.

import { spawn } from "node:child_process";

function runCommand(req: Request, res: Response) {
    const userPath = req.body.path;
    process.env.PATH = userPath;
    const proc = spawn("node", ["script.js"]);
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
