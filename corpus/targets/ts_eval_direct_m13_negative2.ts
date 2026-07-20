// SAFE: Hono with JSON.parse
import { Hono } from "hono";
const app = new Hono();
app.post("/eval", async (c) => { try { const { expression } = await c.req.json(); const result = JSON.parse(expression); return c.json({ result }); } catch { return c.json({ error: "Invalid JSON" }, 400); } });
