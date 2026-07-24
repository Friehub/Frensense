// [frensense]
// observation: User input from req.body.cmd is forwarded through a helper
//              function and reaches exec() via string interpolation.
// impact: An attacker can execute arbitrary OS commands.
// improvement: Use execFile with array arguments.
// cwe: CWE-78
// cvss: 9.8

import { exec } from "child_process";
import express from "express";

const app = express();

function buildCommand(cmd: string): string {
    return `/bin/sh -c "${cmd}"`;
}

app.post("/run", (req: express.Request, res: express.Response) => {
    const cmd = req.body.cmd as string;
    exec(buildCommand(cmd), (err, stdout) => {
        res.json({ output: stdout });
    });
});
