// [frensense]
// observation: User-controlled input flows through an intermediate variable into exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input via the query parameter.
// improvement: Validate the command against an allowlist or use execFile with arguments array.

function handler(req: any, res: any) {
    const cmd = req.query.cmd;
    exec(cmd);
}

function runTask(req: any, res: any) {
    const task = req.body.task;
    exec(task);
}
