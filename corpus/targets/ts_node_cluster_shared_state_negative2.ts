// SAFE: Shared state uses atomic database operations rather than in-memory variables.

import cluster from "node:cluster";
import { cpus } from "node:os";
import { createClient } from "redis";

const redis = createClient();

if (cluster.isPrimary) {
    for (let i = 0; i < cpus().length; i++) cluster.fork();
}

async function incrementCounter(): Promise<number> {
    return await redis.incr("request_count");
}
