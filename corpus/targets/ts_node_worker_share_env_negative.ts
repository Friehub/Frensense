// SAFE: An explicit env object is passed to the Worker, containing only the variables required.

import { Worker } from "node:worker_threads";

function createWorker() {
    const worker = new Worker("./background.js", {
        env: { NODE_ENV: process.env.NODE_ENV ?? "production" },
    });
    worker.postMessage({ task: "process" });
    worker.on("message", msg => console.log(msg));
}
