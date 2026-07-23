// SAFE: User arguments are filtered to strip dangerous Node.js flags before forking.

import { fork } from "node:child_process";

const DANGEROUS_FLAGS = new Set(["--inspect", "--inspect-brk", "--eval", "-e", "--require", "-r", "--experimental-loader"]);

function forkWorker(req: Request, res: Response) {
    const modulePath = "./worker.js";
    const userArgs = req.body.args;
    if (!Array.isArray(userArgs)) {
        return res.status(400).json({ error: "Invalid arguments" });
    }
    const safeArgs = userArgs.filter(a => !DANGEROUS_FLAGS.has(a));
    const child = fork(modulePath, safeArgs, { silent: true });
    child.on("message", msg => res.json(msg));
}
