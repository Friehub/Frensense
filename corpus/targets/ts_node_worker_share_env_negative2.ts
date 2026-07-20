// SAFE: The env option is set to an empty object, preventing any environment variable inheritance.

import { Worker } from "node:worker_threads";

function createWorker() {
    const worker = new Worker("./background.js", { env: {} });
    worker.postMessage({ task: "process" });
    worker.on("message", msg => console.log(msg));
}
