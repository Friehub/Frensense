// SAFE: Hono with parameterized query
import { Hono } from "hono";
const app = new Hono();
app.get("/user/:id", async (c) => { const userId = c.req.param("id"); const result = await db.query("SELECT * FROM users WHERE id = $1", [userId]); return c.json(result.rows[0]); });
app.post("/order/delete", async (c) => { const { orderId } = await c.req.json(); await db.query("DELETE FROM orders WHERE id = $1", [orderId]); return c.json({ success: true }); });
