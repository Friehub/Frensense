// SAFE: The cwd is resolved and verified to be within the allowed base directory.

import { resolve, normalize } from "node:path";
import { spawn } from "node:child_process";

const BASE_DIR = "/home/user/projects";

function runInUserDir(req: Request, res: Response) {
    const userDir = req.body.cwd;
    const resolved = normalize(resolve(userDir));
    if (!resolved.startsWith(BASE_DIR)) {
        return res.status(400).json({ error: "Directory outside allowed base" });
    }
    const proc = spawn("npm", ["install"], { cwd: resolved });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
