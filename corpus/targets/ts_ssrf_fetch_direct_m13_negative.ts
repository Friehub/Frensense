// SAFE: Hono with URL validation
import { Hono } from "hono";
const app = new Hono();
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];
function isValidUrl(url: string): boolean { try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); } catch { return false; } }
app.get("/fetch", async (c) => { const url = c.req.query("url"); if (!isValidUrl(url)) return c.json({ error: "Host not allowed" }, 403); const response = await fetch(url); const data = await response.json(); return c.json(data); });
app.post("/proxy", async (c) => { const { target } = await c.req.json(); if (!isValidUrl(target)) return c.json({ error: "Host not allowed" }, 403); const result = await fetch(target); const body = await result.text(); return c.body(body); });
