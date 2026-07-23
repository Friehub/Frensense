// SAFE: Hono with path.resolve check
import * as fs from "fs";
import * as path from "path";
import { Hono } from "hono";
const app = new Hono();
const BASE_DIR = path.resolve("/var/uploads");
app.get("/files/:filename", (c) => { const filename = c.req.param("filename"); const requested = path.resolve(BASE_DIR, filename); if (!requested.startsWith(BASE_DIR)) return c.text("Invalid path", 403); const content = fs.readFileSync(requested, "utf-8"); return c.text(content); });
