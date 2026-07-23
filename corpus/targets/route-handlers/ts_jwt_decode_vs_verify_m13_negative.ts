// SAFE: Hono with jwt.verify
import jwt from "jsonwebtoken";
import { Hono } from "hono";
import { getCookie } from "hono/cookie";
const app = new Hono();
const SECRET = process.env.JWT_SECRET || "fallback-secret";
app.get("/profile", async (c) => { const auth = c.req.header("Authorization") || ""; try { const payload = jwt.verify(auth, SECRET); return c.json(payload); } catch { return c.json({ error: "Invalid token" }, 401); } });
app.get("/dashboard", async (c) => { const token = getCookie(c, "token") || ""; try { const payload = jwt.verify(token, SECRET); return c.json(payload); } catch { return c.json({ error: "Invalid token" }, 401); } });
