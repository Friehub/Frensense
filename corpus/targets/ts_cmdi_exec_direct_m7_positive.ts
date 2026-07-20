// [frensense]
// observation: User-controlled input is destructured from request object and passed to exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by destructuring crafted input into exec().
// improvement: Validate destructured input against an allowlist or use execFile with arguments.

function handler(req: any, res: any) {
    const { cmd } = req.query;
    exec(cmd);
}

function runTask(req: any, res: any) {
    const { task } = req.body;
    exec(task);
}
