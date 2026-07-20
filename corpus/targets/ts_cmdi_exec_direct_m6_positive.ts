// [frensense]
// observation: User-controlled input is concatenated into a shell command string and passed to exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by injecting shell metacharacters through string concatenation.
// improvement: Validate against an allowlist or use execFile with arguments array.

function handler(req: any, res: any) {
    exec("/usr/bin/" + req.query.cmd);
}

function runTask(req: any, res: any) {
    exec("run-" + req.body.task);
}
