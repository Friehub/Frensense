function handler(req: any, res: any) {
    const allowed = ["ls", "pwd", "date"];
    const a = req.query.cmd;
    const b = a;
    if (allowed.includes(b)) {
        exec(b);
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const input = req.body.task;
    const task = input;
    const command = task;
    if (allowed.includes(command)) {
        exec(command);
    } else {
        res.status(403).send("Task not allowed");
    }
}
