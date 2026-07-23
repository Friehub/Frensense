// [frensense]
// observation: Cluster workers do not handle uncaughtException, causing the entire worker process to terminate and potentially crash the cluster.
// impact: An uncaught exception in any worker takes down that worker process, reducing cluster capacity and potentially causing cascading failures.
// improvement: Add a process-level uncaughtException handler in workers that logs the error and gracefully shuts down, or respawns the worker from the primary.

import cluster from "node:cluster";
import { cpus } from "node:os";
import http from "node:http";

if (cluster.isPrimary) {
    for (let i = 0; i < cpus().length; i++) cluster.fork();
}

if (cluster.isWorker) {
    http.createServer((req, res) => {
        throw new Error("unhandled error");
    }).listen(3000);
}
