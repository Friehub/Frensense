// SAFE: Hono with HTML escaping
import { Hono } from "hono";
import { escape } from "html-escaper";
const app = new Hono();
app.get("/search", (c) => { const query = c.req.query("q") || ""; const safeQ = escape(query); return c.html(`<html><body><h1>Search results for: ${safeQ}</h1></body></html>`); });
app.get("/greet", (c) => { const name = c.req.query("name") || ""; const safeName = escape(name); return c.html(`<p>Welcome, ${safeName}!</p>`); });
