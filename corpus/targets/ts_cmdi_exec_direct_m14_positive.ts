// [frensense]
// observation: User-controlled input is passed to exec() without sanitization with renamed variables.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate against allowlist or use execFile
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";

function handleRequest(req: any, res: any) {
    const userCommand = req.query.cmd;
    exec(userCommand);
}

function processAction(req: any, res: any) {
    const actionName = req.body.task;
    exec(actionName);
}
