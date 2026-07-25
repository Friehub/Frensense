// [frensense]
// observation: Resource ID from URL parameter is used without ownership verification using Hono framework instead of Express.
// impact: An attacker can access other users' resources.
// improvement: Verify authenticated user owns the resource
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

import { Hono } from "hono";
const app = new Hono();

app.get("/invoices/:id", async (c) => { const id = c.req.param("id"); const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(id).first(); if (!invoice) return c.json({ error: "Not found" }, 404); return c.json(invoice); });

app.get("/orders/:orderId", async (c) => { const orderId = c.req.param("orderId"); const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first(); if (!order) return c.json({ error: "Not found" }, 404); return c.json(order); });
