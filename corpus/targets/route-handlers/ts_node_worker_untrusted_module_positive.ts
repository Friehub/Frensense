// [frensense]
// observation: user-controlled input is used as the module path for constructing a Worker, allowing arbitrary files to be executed as worker threads.
// impact: An attacker can specify any JavaScript file on the system as the worker entry point, leading to arbitrary code execution.
// improvement: Validate the worker module path against an allowlist of permitted files, or resolve it to an absolute path and verify it resides within an allowed directory.
// cwe: CWE-829
// cvss: 8.8
// owasp: A06:2021
// severity: High

import { Worker } from "node:worker_threads";

function startUserWorker(req: Request, res: Response) {
    const userPath = req.body.modulePath;
    const worker = new Worker(userPath);
    worker.on("message", msg => res.json(msg));
    worker.on("error", err => res.status(500).json({ error: err.message }));
}
