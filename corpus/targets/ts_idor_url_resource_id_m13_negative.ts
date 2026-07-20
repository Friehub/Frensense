// SAFE: Hono with ownership verification
import { Hono } from "hono";
const app = new Hono();
app.get("/invoices/:id", async (c) => { const id = c.req.param("id"); const userId = c.get("user").id; const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(id, userId).first(); if (!invoice) return c.json({ error: "Not found" }, 404); return c.json(invoice); });
app.get("/orders/:orderId", async (c) => { const orderId = c.req.param("orderId"); const userId = c.get("user").id; const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(orderId, userId).first(); if (!order) return c.json({ error: "Not found" }, 404); return c.json(order); });
