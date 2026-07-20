// [frensense]
// observation: User-controlled input is passed to exec() without sanitization through an object property.
// impact: An attacker can execute arbitrary system commands by supplying crafted input.
// improvement: Validate the command against an allowlist or use execFile with arguments array

import { exec } from "child_process";

function handler(req: any, res: any) {
    const cfg = { command: req.query.cmd };
    exec(cfg.command);
}

function runTask(req: any, res: any) {
    const opts = { action: req.body.task };
    exec(opts.action);
}
