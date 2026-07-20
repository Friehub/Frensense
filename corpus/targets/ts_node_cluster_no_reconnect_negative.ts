// SAFE: Cluster forks a new worker when a worker disconnects, maintaining pool size.

import cluster from "node:cluster";
import { cpus } from "node:os";

if (cluster.isPrimary) {
    const workerCount = cpus().length;
    for (let i = 0; i < workerCount; i++) {
        cluster.fork();
    }
    cluster.on("disconnect", worker => {
        console.log(`Worker ${worker.process.pid} disconnected, forking replacement`);
        cluster.fork();
    });
}
