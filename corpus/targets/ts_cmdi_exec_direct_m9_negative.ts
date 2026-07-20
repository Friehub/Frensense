// SAFE: Object property validated against allowlist
const ALLOWED = ["ls", "pwd", "date"];

function handler(req: any, res: any) {
    const cfg = { command: req.query.cmd };
    if (ALLOWED.includes(cfg.command)) {
        exec(cfg.command);
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req: any, res: any) {
    const allowed = ["build", "deploy", "test"];
    const opts = { action: req.body.task };
    if (allowed.includes(opts.action)) {
        exec(opts.action);
    } else {
        res.status(403).send("Task not allowed");
    }
}
