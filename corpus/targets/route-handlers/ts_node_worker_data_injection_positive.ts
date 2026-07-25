// [frensense]
// observation: unvalidated user data is sent to a Worker via postMessage, potentially allowing arbitrary code execution through the worker's message handling.
// impact: If the worker evaluates or executes received data unsafely (e.g., via eval, new Function, or dynamic require), an attacker can achieve RCE in the worker thread.
// improvement: Validate and sanitize all data before posting to workers, and ensure workers do not evaluate untrusted message content.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { Worker } from "node:worker_threads";

function processInWorker(req: Request, res: Response) {
    const userData = req.body.data;
    const worker = new Worker("./processor.js");
    worker.postMessage(userData);
    worker.on("message", result => res.json(result));
    worker.on("error", err => res.status(500).json({ error: err.message }));
}
