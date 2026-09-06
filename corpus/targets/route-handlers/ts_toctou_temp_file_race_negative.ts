// SAFE: Uses random UUID for temp filenames, making them unpredictable and resistant to symlink race attacks
import express from "express";
import fs from "fs";
import path from "path";
import crypto from "crypto";

export function saveTempFile(req: express.Request, res: express.Response) {
    const tempPath = path.join("/tmp", crypto.randomUUID() + ".tmp");
    fs.writeFileSync(tempPath, req.body.content);
    const data = fs.readFileSync(tempPath, "utf-8");
    res.json({ size: data.length });
}

export function processExport(req: express.Request, res: express.Response) {
    const tempPath = path.join("/tmp", crypto.randomUUID() + ".csv");
    fs.writeFileSync(tempPath, generateCsv());
    res.download(tempPath);
}
