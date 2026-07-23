// SAFE: Conditional branch with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    if (req.query.cmd) {
        const cmd = req.query.cmd;
        if (ALLOWED.includes(cmd)) { exec(cmd); } else { res.status(403).send("Command not allowed"); }
    } else { res.send("No command provided"); }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    if (req.body.task && req.body.task.length > 0) {
        const task = req.body.task;
        if (allowed.includes(task)) { exec(task); } else { res.status(403).send("Task not allowed"); }
    }
}
