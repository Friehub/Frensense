// [frensense]
// observation: A Worker is created but never terminated on timeout or error, causing a resource leak that accumulates over time.
// impact: Worker threads hold libuv handles and memory; failing to terminate them on timeout leads to resource exhaustion and eventual DoS.
// improvement: Set a timeout and call worker.terminate() if the worker does not respond in time.

import { Worker } from "node:worker_threads";

function runWorkerTask(data: unknown): Promise<unknown> {
    return new Promise((resolve, reject) => {
        const worker = new Worker("./task.js");
        worker.postMessage(data);
        worker.on("message", resolve);
        worker.on("error", reject);
    });
}
