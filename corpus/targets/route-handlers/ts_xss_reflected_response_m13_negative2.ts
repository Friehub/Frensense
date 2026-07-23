// SAFE: Hono with encodeURI
import { Hono } from "hono";
const app = new Hono();
app.get("/search", (c) => { const query = c.req.query("q") || ""; return c.html(`<html><body><h1>Search results for: ${encodeURI(query)}</h1></body></html>`); });
