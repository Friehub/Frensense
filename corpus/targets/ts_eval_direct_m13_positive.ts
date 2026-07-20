// [frensense]
// observation: User-controlled input is passed directly to eval using Hono framework instead of Express.
// impact: An attacker can execute arbitrary JavaScript.
// improvement: Avoid eval; use mathjs or JSON.parse

import { Hono } from "hono";
const app = new Hono();

app.post("/eval", async (c) => { const { expression } = await c.req.json(); const result = eval(expression); return c.json({ result }); });
app.get("/exec", async (c) => { const code = c.req.query("code") || ""; const result = eval(code); return c.json({ result }); });
