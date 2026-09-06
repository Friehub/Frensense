// SAFE: The file argument is validated against an allowlist of permitted binaries before spawning.

import { spawn } from "node:child_process";

const ALLOWED_BINARIES = new Set(["/usr/bin/gzip", "/usr/bin/git", "/usr/local/bin/node"]);

function executeUserFile(req: Request, res: Response) {
    const userFile = req.body.file;
    const args = req.body.args ?? [];
    if (!ALLOWED_BINARIES.has(userFile)) {
        return res.status(400).json({ error: "Disallowed binary" });
    }
    const proc = spawn(userFile, args);
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
