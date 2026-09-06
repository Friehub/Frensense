// SAFE: Command is selected from a fixed internal mapping. User input only picks which task runs.
// No user string reaches the shell.

import { execFile } from "child_process";
import express from "express";

const TASKS: Record<string, string> = {
    build: "/usr/local/bin/build",
    test: "/usr/local/bin/test",
    lint: "/usr/local/bin/lint",
};

const app = express();
app.use(express.json());

app.post("/api/task", (req: express.Request, res: express.Response) => {
    const taskName = req.body.taskName as string;
    const bin = TASKS[taskName];
    if (!bin) {
        return res.status(404).json({ error: "Unknown task" });
    }
    execFile(bin, [], { encoding: "utf8" }, (err, stdout) => {
        if (err) return res.status(500).json({ error: "Task failed" });
        res.json({ result: stdout.trim() });
    });
});
