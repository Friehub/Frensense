// [frensense]
// observation: User-controlled input traverses multiple variable assignments before reaching exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input through multiple assignment hops.
// improvement: Validate input against an allowlist or use execFile with arguments.

function handler(req: any, res: any) {
    const a = req.query.cmd;
    const b = a;
    exec(b);
}

function runTask(req: any, res: any) {
    const input = req.body.task;
    const task = input;
    const command = task;
    exec(command);
}
