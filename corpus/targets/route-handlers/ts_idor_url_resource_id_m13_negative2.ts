// SAFE: Hono with ownership check via JOIN
import { Hono } from "hono";
const app = new Hono();
app.get("/invoices/:id", async (c) => { const id = c.req.param("id"); const invoice = await db.prepare('SELECT i.* FROM invoices i JOIN users u ON i.user_id = u.id WHERE i.id = ? AND u.id = ?').bind(id, c.get("user").id).first(); if (!invoice) return c.json({ error: "Not found" }, 404); return c.json(invoice); });
