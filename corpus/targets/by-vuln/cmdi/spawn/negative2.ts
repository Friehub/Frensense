// SAFE: shell: true is not used. Binary is hardcoded. User input is ignored for the command path.

import { spawn } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

app.post("/api/run", (req: express.Request, res: express.Response) => {
    const child = spawn("/usr/local/bin/worker", ["--task", "default"], {
        stdio: "pipe",
    });
    child.stdout.pipe(res);
});
