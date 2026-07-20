// SAFE: The worker path is resolved to an absolute path and verified to be within the workers directory.

import { resolve, normalize } from "node:path";
import { Worker } from "node:worker_threads";

const WORKERS_BASE = normalize(resolve("./workers"));

function startUserWorker(req: Request, res: Response) {
    const userPath = req.body.modulePath;
    const resolved = normalize(resolve(userPath));
    if (!resolved.startsWith(WORKERS_BASE)) {
        return res.status(400).json({ error: "Worker path outside allowed directory" });
    }
    const worker = new Worker(resolved);
    worker.on("message", msg => res.json(msg));
    worker.on("error", err => res.status(500).json({ error: err.message }));
}
