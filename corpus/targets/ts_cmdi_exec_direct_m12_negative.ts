// SAFE: Try-catch with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    try {
        const cmd = req.query.cmd;
        if (!ALLOWED.includes(cmd)) return res.status(403).send("Command not allowed");
        exec(cmd);
    } catch (err) { console.error(err); res.status(500).send("Error"); }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    try {
        const task = req.body.task;
        if (!allowed.includes(task)) return res.status(403).send("Task not allowed");
        exec(task);
    } catch (err) { console.error(err); }
}
