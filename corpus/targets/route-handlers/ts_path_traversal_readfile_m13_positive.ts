// [frensense]
// observation: User-controlled path is passed to readFileSync without directory traversal prevention using Hono framework instead of Express.
// impact: An attacker can read arbitrary files.
// improvement: Use path.basename() and verify resolved path

import * as fs from "fs";
import * as path from "path";
import { Hono } from "hono";
const app = new Hono();

app.get("/files/:filename", (c) => { const filename = c.req.param("filename"); const filePath = path.join("/var/uploads", filename); const content = fs.readFileSync(filePath, "utf-8"); return c.text(content); });

app.get("/assets", (c) => { const assetPath = c.req.query("path") || ""; const fullPath = path.join("/var/static", assetPath); const data = fs.readFileSync(fullPath); return c.body(data); });
