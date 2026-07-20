// SAFE: The file path is resolved to an absolute path and verified to reside within a permitted directory.

import { resolve, normalize } from "node:path";
import { spawn } from "node:child_process";

const ALLOWED_DIR = "/usr/local/bin";

function executeUserFile(req: Request, res: Response) {
    const userFile = req.body.file;
    const args = req.body.args ?? [];
    const resolved = normalize(resolve(userFile));
    if (!resolved.startsWith(ALLOWED_DIR)) {
        return res.status(400).json({ error: "Disallowed binary path" });
    }
    const proc = spawn(resolved, args);
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
