// [frensense]
// observation: spawn is called with shell: true and user-controlled input from req.body.cmd, executing arbitrary shell commands despite using spawn instead of exec.
// impact: Setting shell: true makes spawn behave like exec — shell metacharacters in the input enable arbitrary command execution.
// improvement: Avoid shell: true when passing user input. Pass arguments as an array to the spawned process.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { spawn } from "child_process";
import express from "express";

const app = express();
app.use(express.json());

function getCommand(req: express.Request): string {
    return req.body.cmd as string;
}

app.post("/api/run", (req: express.Request, res: express.Response) => {
    const cmd = getCommand(req);
    const child = spawn(cmd, [], { shell: true, stdio: "pipe" });
    child.stdout.on("data", (d: Buffer) => res.write(d));
    child.on("close", () => res.end());
});
