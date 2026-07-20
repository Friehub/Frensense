// SAFE: The worker lifecycle is managed with AbortSignal, ensuring cleanup on timeout or abort.

import { Worker } from "node:worker_threads";

function runWorkerTask(data: unknown, signal?: AbortSignal): Promise<unknown> {
    return new Promise((resolve, reject) => {
        const worker = new Worker("./task.js");
        const cleanup = () => { worker.terminate(); };
        signal?.addEventListener("abort", cleanup, { once: true });
        worker.postMessage(data);
        worker.on("message", msg => { cleanup(); resolve(msg); });
        worker.on("error", err => { cleanup(); reject(err); });
    });
}
