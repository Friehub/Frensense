const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    const { cmd } = req.query;
    if (ALLOWED.includes(cmd)) {
        exec(cmd);
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const { task } = req.body;
    if (allowed.includes(task)) {
        exec(task);
    } else {
        res.status(403).send("Task not allowed");
    }
}
