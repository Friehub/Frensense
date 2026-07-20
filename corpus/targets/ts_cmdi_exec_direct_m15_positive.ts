// [frensense]
// observation: User-controlled input is passed to exec() without sanitization via a promise .then() chain.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate against allowlist or use execFile

import { exec } from "child_process";

function handler(req: any, res: any) {
    Promise.resolve(req.query.cmd).then(cmd => { exec(cmd); });
}

function runTask(req: any, res: any) {
    new Promise(resolve => resolve(req.body.task)).then(task => { exec(task); });
}
