// SAFE: Workers have an uncaughtException handler that logs the error and exits gracefully, allowing the primary to respawn.

import cluster from "node:cluster";
import { cpus } from "node:os";
import http from "node:http";

if (cluster.isPrimary) {
    for (let i = 0; i < cpus().length; i++) cluster.fork();
    cluster.on("exit", (worker, code, signal) => {
        console.log(`Worker ${worker.process.pid} died, forking replacement`);
        cluster.fork();
    });
}

if (cluster.isWorker) {
    process.on("uncaughtException", err => {
        console.error("Uncaught exception:", err);
        process.exit(1);
    });
    http.createServer((req, res) => {
        throw new Error("unhandled error");
    }).listen(3000);
}
