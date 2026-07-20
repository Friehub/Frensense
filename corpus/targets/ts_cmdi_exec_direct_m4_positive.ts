// [frensense]
// observation: User-controlled input is passed through a helper function before reaching exec() without sanitization in the helper.
// impact: An attacker can execute arbitrary system commands by supplying crafted input through an unsafe helper function.
// improvement: Validate input inside the helper or use execFile with arguments.

function getCommand(req: any): string {
    return req.query.cmd;
}

function getTask(req: any): string {
    return req.body.task;
}

function handler(req: any, res: any) {
    const cmd = getCommand(req);
    exec(cmd);
}

function runTask(req: any, res: any) {
    const task = getTask(req);
    exec(task);
}
