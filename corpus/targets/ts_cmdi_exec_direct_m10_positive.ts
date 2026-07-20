// [frensense]
// observation: User-controlled input is passed to exec() without sanitization across an async/await boundary.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate the command against an allowlist or use execFile

import { exec } from "child_process";

async function getCommand(req: any): Promise<string> { return req.query.cmd; }
async function getTask(req: any): Promise<string> { return req.body.task; }

async function handler(req: any, res: any) {
    const cmd = await getCommand(req);
    exec(cmd);
}

async function runTask(req: any, res: any) {
    const task = await getTask(req);
    exec(task);
}
