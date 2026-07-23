// SAFE: Fastify with allowlist and execFile
import { exec } from "child_process";
import Fastify from "fastify";
import { execFile } from "child_process";
const app = Fastify();
const ALLOWED = new Set(["ls", "pwd", "date"]);

app.get("/exec", async (req, reply) => { const cmd = req.query.cmd; if (!ALLOWED.has(cmd)) return reply.status(403).send("Not allowed"); return new Promise((resolve) => { execFile(cmd, [], (error, stdout) => { if (error) return resolve(reply.status(500).send("Error")); resolve(reply.send(stdout)); }); }); });
