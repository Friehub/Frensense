// [frensense]
// observation: user-controlled input is used as the cwd option for child_process.spawn, enabling path confusion attacks where the working directory can be changed to a malicious location.
// impact: An attacker can set cwd to a directory containing a malicious executable that shadows the intended command, leading to arbitrary code execution.
// improvement: Validate the cwd path against an allowlist of permitted directories, or resolve it and verify it falls within an allowed base path.

import { spawn } from "node:child_process";

function runInUserDir(req: Request, res: Response) {
    const userDir = req.body.cwd;
    const proc = spawn("npm", ["install"], { cwd: userDir });
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
