function handler(req: any, res: any) {
    const allowed = ["ls", "pwd", "date"];
    const cmd = req.query.cmd;
    if (allowed.includes(cmd)) {
        exec(cmd);
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const task = req.body.task;
    if (allowed.includes(task)) {
        exec(task);
    } else {
        res.status(403).send("Task not allowed");
    }
}
