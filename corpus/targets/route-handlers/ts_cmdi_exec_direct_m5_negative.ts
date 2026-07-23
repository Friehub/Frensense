const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    const cmd = req.query.cmd;
    if (ALLOWED.includes(cmd)) {
        exec(`/usr/bin/${cmd}`);
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const task = req.body.task;
    if (allowed.includes(task)) {
        exec(`run-${task}`);
    } else {
        res.status(403).send("Task not allowed");
    }
}
