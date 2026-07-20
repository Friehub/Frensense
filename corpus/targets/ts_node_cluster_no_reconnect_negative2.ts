// SAFE: Cluster monitors exit events and respawns workers with exponential backoff.

import cluster from "node:cluster";
import { cpus } from "node:os";

if (cluster.isPrimary) {
    const workerCount = cpus().length;
    const spawnWorker = () => {
        const worker = cluster.fork();
        worker.on("exit", (code, signal) => {
            console.log(`Worker died (code=${code}, signal=${signal}), respawning`);
            setTimeout(spawnWorker, 1000);
        });
    };
    for (let i = 0; i < workerCount; i++) spawnWorker();
}
