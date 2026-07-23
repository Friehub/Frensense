// [frensense]
// observation: User-controlled input is passed to exec() without sanitization inside a conditional block on the tainted branch.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate against an allowlist or use execFile

import { exec } from "child_process";

function handler(req: any, res: any) {
    if (req.query.cmd) {
        exec(req.query.cmd);
    } else {
        res.send("No command provided");
    }
}

function runTask(req: any, res: any) {
    if (req.body.task && req.body.task.length > 0) {
        exec(req.body.task);
    }
}
