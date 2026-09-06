// SAFE: Uses mkdtemp to create a dedicated directory per operation, ensuring isolation and preventing race conditions
import express from "express";
import fs from "fs";
import path from "path";

export function saveTempFile(req: express.Request, res: express.Response) {
    const tmpDir = fs.mkdtempSync("/tmp/upload-");
    const tempPath = path.join(tmpDir, "content.tmp");
    fs.writeFileSync(tempPath, req.body.content);
    const data = fs.readFileSync(tempPath, "utf-8");
    res.json({ size: data.length });
}

export function processExport(req: express.Request, res: express.Response) {
    const tmpDir = fs.mkdtempSync("/tmp/export-");
    const tempPath = path.join(tmpDir, "data.csv");
    fs.writeFileSync(tempPath, generateCsv());
    res.download(tempPath);
}
