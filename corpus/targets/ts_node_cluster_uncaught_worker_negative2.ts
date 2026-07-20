// SAFE: Workers use a domain or try-catch at the request level and notify the primary on crash.

import cluster from "node:cluster";
import { cpus } from "node:os";
import http from "node:http";
import domain from "node:domain";

if (cluster.isPrimary) {
    for (let i = 0; i < cpus().length; i++) cluster.fork();
    cluster.on("exit", (worker, code, signal) => {
        if (code !== 0) cluster.fork();
    });
}

if (cluster.isWorker) {
    http.createServer((req, res) => {
        const d = domain.create();
        d.on("error", err => {
            console.error("Request error:", err);
            res.statusCode = 500;
            res.end("Internal error");
        });
        d.run(() => {
            throw new Error("unhandled error");
        });
    }).listen(3000);
}
