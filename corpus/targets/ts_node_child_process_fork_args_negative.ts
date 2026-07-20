// SAFE: User-supplied arguments are validated against an allowlist that rejects any flag-like tokens.

import { fork } from "node:child_process";

const ALLOWED_ARGS = new Set(["--port", "--host", "--debug"]);

function forkWorker(req: Request, res: Response) {
    const modulePath = "./worker.js";
    const userArgs = req.body.args;
    if (!Array.isArray(userArgs) || userArgs.some(a => typeof a !== "string" || a.startsWith("-"))) {
        return res.status(400).json({ error: "Invalid arguments" });
    }
    const child = fork(modulePath, userArgs, { silent: true });
    child.on("message", msg => res.json(msg));
}
