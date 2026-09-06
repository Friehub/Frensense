// SAFE: The filename is sanitized to only allow alphanumeric and safe characters; path is resolved and prefix-checked
import express from "express";
import fs from "fs";
import path from "path";

const DATA_DIR = path.resolve("/var/data");
const LOG_DIR = path.resolve("/var/logs");

function safeBasename(name: string): string {
    return path.basename(name).replace(/[^a-zA-Z0-9._-]/g, "_");
}

export function saveFile(req: express.Request, res: express.Response) {
    const filename = safeBasename(req.body.filename);
    const content = req.body.content;
    const filePath = path.resolve(DATA_DIR, filename);
    if (!filePath.startsWith(DATA_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    fs.writeFileSync(filePath, content);
    res.json({ success: true });
}

export function appendFile(req: express.Request, res: express.Response) {
    const logFile = safeBasename(req.query.file as string);
    const entry = req.body.entry;
    const filePath = path.resolve(LOG_DIR, logFile);
    if (!filePath.startsWith(LOG_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    fs.appendFileSync(filePath, entry + "\n");
    res.json({ success: true });
}
