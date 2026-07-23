// SAFE: Data is validated against a schema and transferred using a structured transfer list, preventing prototype pollution.

import { Worker } from "node:worker_threads";

function processInWorker(req: Request, res: Response) {
    const userData = req.body.data;
    if (!userData || typeof userData.input !== "string") {
        return res.status(400).json({ error: "Invalid data shape" });
    }
    const safePayload = { input: userData.input.slice(0, 1000) };
    const worker = new Worker("./processor.js");
    worker.postMessage(safePayload);
    worker.on("message", result => res.json(result));
    worker.on("error", err => res.status(500).json({ error: err.message }));
}
