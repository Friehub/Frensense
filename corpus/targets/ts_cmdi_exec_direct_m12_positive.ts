// [frensense]
// observation: User-controlled input is passed to exec() without sanitization inside a try-catch block.
// impact: An attacker can execute arbitrary system commands, with errors silently caught.
// improvement: Validate against allowlist or use execFile

import { exec } from "child_process";

function handler(req: any, res: any) {
    try { exec(req.query.cmd); } catch (err) { console.error(err); }
}

function runTask(req: any, res: any) {
    try { exec(req.body.task); } catch {}
}
