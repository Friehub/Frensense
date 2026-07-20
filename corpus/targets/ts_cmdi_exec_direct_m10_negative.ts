// SAFE: Async path with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["ls", "pwd", "date"];

async function getCommand(req: any): Promise<string> {
    const cmd = req.query.cmd;
    if (!ALLOWED.includes(cmd)) throw new Error("Not allowed");
    return cmd;
}

async function getTask(req: any): Promise<string> {
    const allowed = ["build", "deploy", "test"];
    const task = req.body.task;
    if (!allowed.includes(task)) throw new Error("Not allowed");
    return task;
}

async function handler(req: any, res: any) {
    try { const cmd = await getCommand(req); exec(cmd); } catch { res.status(403).send("Command not allowed"); }
}

async function runTask(req: any, res: any) {
    try { const task = await getTask(req); exec(task); } catch { res.status(403).send("Task not allowed"); }
}
