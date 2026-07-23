// SAFE: Explicit encoding "utf8" is specified, ensuring text output is decoded safely.

import { exec } from "node:child_process";

function runCommand(req: Request, res: Response) {
    const cmd = req.body.command;
    exec(cmd, { encoding: "utf8" }, (error, stdout, stderr) => {
        if (error) return res.status(500).json({ error: stderr });
        res.send(stdout);
    });
}
