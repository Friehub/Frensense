const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    const cmds = [req.query.cmd];
    if (ALLOWED.includes(cmds[0])) {
        exec(cmds[0]);
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const tasks = [req.body.task];
    if (allowed.includes(tasks[0])) {
        exec(tasks[0]);
    } else {
        res.status(403).send("Task not allowed");
    }
}
