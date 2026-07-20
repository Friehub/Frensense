// SAFE: Absolute paths are used for binaries instead of relying on PATH resolution.

import { spawn } from "node:child_process";

function runCommand(req: Request, res: Response) {
    const proc = spawn("/usr/local/bin/node", ["script.js"]);
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
