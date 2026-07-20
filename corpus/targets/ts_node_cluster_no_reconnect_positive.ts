// [frensense]
// observation: Cluster workers that disconnect are not reconnected, silently reducing cluster capacity without recovery
// impact: Disconnected workers degrade performance and may cause request queuing or complete service failure under load
// improvement: Listen for the disconnect event and fork a replacement worker, or use the exit event to respawn

import cluster from "node:cluster";
import { cpus } from "node:os";

function startCluster(): void {
    if (cluster.isPrimary) {
        for (let i = 0; i < cpus().length; i++) {
            cluster.fork();
        }
    }
}
