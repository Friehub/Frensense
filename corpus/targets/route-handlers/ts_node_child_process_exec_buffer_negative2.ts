// SAFE: execFile is used with explicit encoding and a validated command path.

import { execFile } from "node:child_process";

function runCommand(req: Request, res: Response) {
    const cmd = req.body.command;
    execFile(cmd, [], { encoding: "utf8" }, (error, stdout, stderr) => {
        if (error) return res.status(500).json({ error: stderr });
        res.send(stdout);
    });
}
