// SAFE: Fastify with execFile
import { exec } from "child_process";
import Fastify from "fastify";
import { execFile } from "child_process";
const app = Fastify();
const ALLOWED = new Set(["convert"]);
app.post("/convert", (req, reply) => { const { filename } = req.body as any; if (!ALLOWED.has(filename)) return reply.status(403).send({ error: "Not allowed" }); execFile("convert", [filename, "-resize", "800x800", "output.jpg"], (err, stdout) => { if (err) return reply.status(500).send({ error: err.message }); return reply.send({ output: stdout }); }); });
