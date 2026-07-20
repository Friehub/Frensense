// [frensense]
// observation: Cluster workers share a mutable in-memory state object without synchronization, causing race conditions.
// impact: Concurrent access from multiple workers leads to inconsistent state, lost updates, and data corruption.
// improvement: Use atomic operations via a shared store (e.g., Redis, database), or use cluster message passing to coordinate state changes.

import cluster from "node:cluster";
import { cpus } from "node:os";

let requestCount = 0;

if (cluster.isPrimary) {
    for (let i = 0; i < cpus().length; i++) cluster.fork();
} else {
    cluster.worker.on("message", () => {
        requestCount++;
    });
}
