// SAFE: .then() chain with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    Promise.resolve(req.query.cmd).then(cmd => {
        if (ALLOWED.includes(cmd)) { exec(cmd); } else { res.status(403).send("Command not allowed"); }
    });
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    new Promise(resolve => resolve(req.body.task)).then(task => {
        if (allowed.includes(task)) { exec(task); } else { res.status(403).send("Task not allowed"); }
    });
}
