// SAFE: Hono with path sanitization
import * as fs from "fs";
import * as path from "path";
import { Hono } from "hono";
const app = new Hono();
const BASE_DIR = "/var/uploads";
app.get("/files/:filename", (c) => { const filename = c.req.param("filename"); const safeName = path.basename(filename); const filePath = path.join(BASE_DIR, safeName); if (!filePath.startsWith(BASE_DIR)) return c.text("Invalid path", 403); const content = fs.readFileSync(filePath, "utf-8"); return c.text(content); });
app.get("/assets", (c) => { const assetPath = c.req.query("path") || ""; const safeName = path.basename(assetPath); const fullPath = path.join(BASE_DIR, safeName); if (!fullPath.startsWith(BASE_DIR)) return c.text("Invalid path", 403); const data = fs.readFileSync(fullPath); return c.body(data); });
