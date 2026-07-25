// [frensense]
// observation: User-controlled input is passed to exec() without sanitization using Fastify framework instead of Express.
// impact: An attacker can execute arbitrary system commands.
// improvement: Validate against allowlist or use execFile
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import Fastify from "fastify";

const app = Fastify();

app.get("/exec", async (req, reply) => {
    const cmd = req.query.cmd; exec(cmd);
    return { executed: true };
});

app.post("/task", async (req, reply) => {
    const { task } = req.body as any; exec(task);
    return { executed: true };
});
