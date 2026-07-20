const ALLOWED_COMMANDS = ["ls", "pwd", "date"];
const ALLOWED_TASKS = ["build", "deploy", "test"];

function getCommand(req: any): string {
    const cmd = req.query.cmd;
    if (!ALLOWED_COMMANDS.includes(cmd)) {
        throw new Error("Command not allowed");
    }
    return cmd;
}

function getTask(req: any): string {
    const task = req.body.task;
    if (!ALLOWED_TASKS.includes(task)) {
        throw new Error("Task not allowed");
    }
    return task;
}

function handler(req: any, res: any) {
    try {
        const cmd = getCommand(req);
        exec(cmd);
    } catch (e) {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    try {
        const task = getTask(req);
        exec(task);
    } catch (e) {
        res.status(403).send("Task not allowed");
    }
}
