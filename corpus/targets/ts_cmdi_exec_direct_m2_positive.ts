// [frensense]
// observation: User-controlled input flows through an intermediate variable into exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input via the query parameter.
// improvement: Validate the command against an allowlist or use execFile with arguments array.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function handler(req: any, res: any) {
    const cmd = req.query.cmd;
    exec(cmd);
}

function runTask(req: any, res: any) {
    const task = req.body.task;
    exec(task);
}
