// [frensense]
// observation: User-controlled input is interpolated into a shell command string using Fastify framework instead of Express.
// impact: An attacker can inject shell metacharacters.
// improvement: Use spawn without shell:true
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import { exec } from "child_process";
import Fastify from "fastify";
const app = Fastify();

app.post("/convert", (req, reply) => { const { filename } = req.body as any; exec(`convert ${filename} -resize 800x800 output.jpg`); return reply.send({ output: "done" }); });

app.post("/clone", (req, reply) => { const { url, destination } = req.body as any; exec(`git clone ${url} /repos/${destination}`); return reply.send({ message: "Cloned successfully" }); });
