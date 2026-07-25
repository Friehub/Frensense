// [frensense]
// observation: User-controlled input is accessed through an array element and passed to exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by crafting array-indexed input.
// improvement: Validate array element against an allowlist or use execFile with arguments.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function handler(req: any, res: any) {
    const cmds = [req.query.cmd];
    exec(cmds[0]);
}

function runTask(req: any, res: any) {
    const tasks = [req.body.task];
    exec(tasks[0]);
}
