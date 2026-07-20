// SAFE: Hono with jwt.verify and Bearer prefix
import jwt from "jsonwebtoken";
import { Hono } from "hono";
const app = new Hono();
const SECRET = process.env.JWT_SECRET || "fallback-secret";
app.get("/profile", async (c) => { try { const auth = c.req.header("Authorization") || ""; const token = auth.replace("Bearer ", ""); const payload = jwt.verify(token, SECRET); return c.json(payload); } catch { return c.json({ error: "Invalid token" }, 401); } });
