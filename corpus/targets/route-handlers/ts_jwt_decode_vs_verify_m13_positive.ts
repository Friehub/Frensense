// [frensense]
// observation: JWT token is decoded without verifying the signature using Hono framework instead of Express.
// impact: An attacker can craft arbitrary JWTs.
// improvement: Always use jwt.verify() instead of jwt.decode()

import jwt from "jsonwebtoken";
import { Hono } from "hono";
import { getCookie } from "hono/cookie";

const app = new Hono();

app.get("/profile", async (c) => { const auth = c.req.header("Authorization") || ""; const payload = jwt.decode(auth); return c.json(payload); });
app.get("/dashboard", async (c) => { const token = getCookie(c, "token") || ""; const payload = jwt.decode(token); return c.json(payload); });
