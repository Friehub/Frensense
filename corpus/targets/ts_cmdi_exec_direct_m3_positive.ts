// [frensense]
// observation: User-controlled input traverses multiple variable assignments before reaching exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input through multiple assignment hops.
// improvement: Validate input against an allowlist or use execFile with arguments.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

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
