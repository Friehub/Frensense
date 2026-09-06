// [frensense]
// observation: user-controlled arguments are passed directly to child_process.fork() as the args array, enabling argv-based RCE through malicious flag injection.
// impact: An attacker can inject Node.js flags like --inspect, --eval, or --require to execute arbitrary code in the forked process.
// improvement: Validate all user-supplied arguments against an allowlist of permitted values, and reject any flag-like arguments.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import { fork } from "node:child_process";

function forkWorker(req: Request, res: Response) {
    const modulePath = "./worker.js";
    const userArgs = req.body.args;
    const child = fork(modulePath, userArgs, { silent: true });
    child.on("message", msg => res.json(msg));
}
