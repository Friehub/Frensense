// [frensense]
// observation: User-controlled input is passed to exec() without sanitization with renamed variables.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate against allowlist or use execFile

import { exec } from "child_process";

function handleRequest(req: any, res: any) {
    const userCommand = req.query.cmd;
    exec(userCommand);
}

function processAction(req: any, res: any) {
    const actionName = req.body.task;
    exec(actionName);
}
