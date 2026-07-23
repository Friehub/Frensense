// SAFE: Hono with parameterized query (alternate)
import { Hono } from "hono";
const app = new Hono();
app.get("/user/:id", async (c) => { const userId = c.req.param("id"); const result = await db.query("SELECT * FROM users WHERE id = $1", [userId]); return c.json(result.rows[0]); });
