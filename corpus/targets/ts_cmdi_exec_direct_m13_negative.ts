// SAFE: Fastify with allowlist validation
import { exec } from "child_process";
import Fastify from "fastify";
const app = Fastify();
const ALLOWED = ["ls", "pwd", "date"];

app.get("/exec", async (req, reply) => { const cmd = req.query.cmd; if (!ALLOWED.includes(cmd)) return reply.status(403).send("Command not allowed"); exec(cmd); return { executed: true }; });

app.post("/task", async (req, reply) => { const allowed = ["build", "deploy", "test"]; const { task } = req.body as any; if (!allowed.includes(task)) return reply.status(403).send("Task not allowed"); exec(task); return { executed: true }; });
