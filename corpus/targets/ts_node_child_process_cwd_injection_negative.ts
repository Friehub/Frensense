// SAFE: The cwd path is validated against an allowlist of permitted directories before use.

import { spawn } from "node:child_process";

const ALLOWED_CWD = new Set(["/home/user/projects", "/var/www/app", "/tmp/builds"]);

function runInUserDir(req: Request, res: Response) {
    const userDir = req.body.cwd;
    if (!ALLOWED_CWD.has(userDir)) {
        return res.status(400).json({ error: "Disallowed directory" });
    }
    const proc = spawn("npm", ["install"], { cwd: userDir });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
