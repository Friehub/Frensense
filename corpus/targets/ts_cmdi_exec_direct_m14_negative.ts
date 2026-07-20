// SAFE: Renamed variables with allowlist validation
import { exec } from "child_process";
const ALLOWED = ["ls", "pwd", "date"];

function handleRequest(req: any, res: any) {
    const userCommand = req.query.cmd;
    if (ALLOWED.includes(userCommand)) { exec(userCommand); } else { res.status(403).send("Command not allowed"); }
}

function processAction(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const actionName = req.body.task;
    if (allowed.includes(actionName)) { exec(actionName); } else { res.status(403).send("Task not allowed"); }
}
