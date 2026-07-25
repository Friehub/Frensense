// [frensense]
// observation: A Worker is created without specifying the env option, causing it to inherit the parent process's environment variables, including secrets like API keys and database credentials.
// impact: The worker thread has full access to all parent process environment variables, potentially exposing secrets to unauthorized code within the worker.
// improvement: Pass an explicit env object containing only the variables the worker actually needs.
// cwe: CWE-526
// cvss: 5.3
// owasp: A02:2021
// severity: Medium

import { Worker } from "node:worker_threads";

function createWorker() {
    const worker = new Worker("./background.js");
    worker.postMessage({ task: "process" });
    worker.on("message", msg => console.log(msg));
}
