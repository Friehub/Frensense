// [frensense]
// observation: User-controlled input is directly interpolated into HTML response without escaping using Hono framework instead of Express.
// impact: An attacker can inject arbitrary HTML/JavaScript (XSS).
// improvement: Encode all user input before embedding in HTML

import { Hono } from "hono";
const app = new Hono();

app.get("/search", (c) => { const query = c.req.query("q") || ""; return c.html(`<html><body><h1>Search results for: ${query}</h1></body></html>`); });

app.get("/greet", (c) => { const name = c.req.query("name") || ""; return c.html(`<p>Welcome, ${name}!</p>`); });
