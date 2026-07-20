// [frensense]
// observation: User-controlled input is accessed through an array element and passed to exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by crafting array-indexed input.
// improvement: Validate array element against an allowlist or use execFile with arguments.

function handler(req: any, res: any) {
    const cmds = [req.query.cmd];
    exec(cmds[0]);
}

function runTask(req: any, res: any) {
    const tasks = [req.body.task];
    exec(tasks[0]);
}
