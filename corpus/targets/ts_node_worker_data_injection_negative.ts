// SAFE: User data is validated and sanitized before being posted to the worker.

import { Worker } from "node:worker_threads";

function processInWorker(req: Request, res: Response) {
    const userData = req.body.data;
    if (typeof userData !== "object" || userData === null) {
        return res.status(400).json({ error: "Invalid data" });
    }
    const sanitized = JSON.parse(JSON.stringify(userData));
    const worker = new Worker("./processor.js");
    worker.postMessage(sanitized);
    worker.on("message", result => res.json(result));
    worker.on("error", err => res.status(500).json({ error: err.message }));
}
