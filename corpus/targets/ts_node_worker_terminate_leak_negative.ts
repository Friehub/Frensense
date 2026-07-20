// SAFE: A timeout is set and worker.terminate() is called if the worker does not respond in time.

import { Worker } from "node:worker_threads";

function runWorkerTask(data: unknown): Promise<unknown> {
    return new Promise((resolve, reject) => {
        const worker = new Worker("./task.js");
        const timer = setTimeout(() => {
            worker.terminate();
            reject(new Error("Worker timeout"));
        }, 30_000);
        worker.postMessage(data);
        worker.on("message", msg => {
            clearTimeout(timer);
            resolve(msg);
        });
        worker.on("error", err => {
            clearTimeout(timer);
            reject(err);
        });
    });
}
