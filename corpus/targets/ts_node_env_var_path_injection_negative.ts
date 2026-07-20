// SAFE: User input is validated against an allowlist before being prepended to PATH.

import { spawn } from "node:child_process";

const ALLOWED_PATH_PREFIXES = ["/home/user/bin", "/opt/custom/bin"];

function runCommand(req: Request, res: Response) {
    const userPath = req.body.path;
    if (!ALLOWED_PATH_PREFIXES.includes(userPath)) {
        return res.status(400).json({ error: "Disallowed PATH entry" });
    }
    process.env.PATH = `${userPath}:${process.env.PATH}`;
    const proc = spawn("node", ["script.js"]);
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
