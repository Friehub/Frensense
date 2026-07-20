// [frensense]
// observation: user-controlled input is passed as the file argument to child_process.spawn() without path validation, enabling arbitrary binary execution.
// impact: An attacker can specify any executable path on the system, leading to arbitrary command execution.
// improvement: Validate the file path against an allowlist of permitted executables, or resolve it to an absolute path and verify it resides within an allowed directory.

import { spawn } from "node:child_process";

function executeUserFile(req: Request, res: Response) {
    const userFile = req.body.file;
    const args = req.body.args ?? [];
    const proc = spawn(userFile, args);
    let output = "";
    proc.stdout.on("data", d => output += d);
    proc.on("close", code => res.json({ code, output }));
}
