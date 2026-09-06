// SAFE: The worker path is validated against an allowlist of permitted module files.

import { Worker } from "node:worker_threads";

const ALLOWED_WORKERS = new Set(["./workers/processor.js", "./workers/analyzer.js", "./workers/renderer.js"]);

function startUserWorker(req: Request, res: Response) {
    const userPath = req.body.modulePath;
    if (!ALLOWED_WORKERS.has(userPath)) {
        return res.status(400).json({ error: "Disallowed worker module" });
    }
    const worker = new Worker(userPath);
    worker.on("message", msg => res.json(msg));
    worker.on("error", err => res.status(500).json({ error: err.message }));
}
