// SAFE: Shared state is managed through message passing to the primary process, ensuring serialized updates.

import cluster from "node:cluster";
import { cpus } from "node:os";

if (cluster.isPrimary) {
    let requestCount = 0;
    for (let i = 0; i < cpus().length; i++) cluster.fork();
    cluster.on("message", (worker, msg) => {
        if (msg.type === "increment") requestCount++;
    });
} else {
    cluster.worker.send({ type: "increment" });
}
