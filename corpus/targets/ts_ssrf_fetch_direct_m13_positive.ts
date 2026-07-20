// [frensense]
// observation: User-controlled URL is passed to fetch() without validation using Hono framework instead of Express.
// impact: An attacker can make the server send requests to internal services.
// improvement: Validate URL against allowlist

import { Hono } from "hono";
const app = new Hono();

app.get("/fetch", async (c) => { const url = c.req.query("url"); const response = await fetch(url); const data = await response.json(); return c.json(data); });
app.post("/proxy", async (c) => { const { target } = await c.req.json(); const result = await fetch(target); const body = await result.text(); return c.body(body); });
