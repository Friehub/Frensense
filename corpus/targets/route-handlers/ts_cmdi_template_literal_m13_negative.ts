// SAFE: Fastify with allowlist and spawn
import { exec } from "child_process";
import Fastify from "fastify";
import { spawn } from "child_process";
const app = Fastify();
const ALLOWED = ["convert", "git"];
app.post("/convert", (req, reply) => { const { filename } = req.body as any; if (!ALLOWED.includes(filename)) return reply.status(403).send({ error: "Not allowed" }); const child = spawn("convert", [filename, "-resize", "800x800", "output.jpg"]); child.stdout.on("data", data => reply.send({ output: data.toString() })); });
