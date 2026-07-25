// [frensense]
// observation: User-controlled input is embedded into a shell command via template literal then passed to exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by injecting shell metacharacters through the template literal.
// improvement: Avoid shell interpolation — validate against an allowlist or use execFile.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function handler(req: any, res: any) {
    exec(`/usr/bin/${req.query.cmd}`);
}

function runTask(req: any, res: any) {
    exec(`run-${req.body.task}`);
}
