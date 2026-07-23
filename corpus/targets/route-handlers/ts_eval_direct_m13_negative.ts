// SAFE: Hono with mathjs evaluate
import { Hono } from "hono";
import math from "mathjs";
const app = new Hono();
app.post("/eval", async (c) => { const { expression } = await c.req.json(); const result = math.evaluate(expression); return c.json({ result }); });
app.get("/exec", async (c) => { const code = c.req.query("code") || ""; const result = math.evaluate(code); return c.json({ result }); });
