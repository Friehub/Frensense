// SAFE: Hono with URL allowlist set
import { Hono } from "hono";
const app = new Hono();
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);
app.get("/fetch", async (c) => { try { const url = c.req.query("url"); const parsed = new URL(url); if (!ALLOWED_HOSTS.has(parsed.hostname)) return c.json({ error: "Host not allowed" }, 403); const response = await fetch(url); const data = await response.json(); return c.json(data); } catch { return c.json({ error: "Invalid URL" }, 400); } });
