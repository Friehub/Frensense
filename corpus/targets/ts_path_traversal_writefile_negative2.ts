// SAFE: Only uses server-generated filenames; user input is never used as a path component
import express from "express";
import fs from "fs";
import path from "path";
import crypto from "crypto";

export function saveFile(req: express.Request, res: express.Response) {
    const content = req.body.content;
    const filename = crypto.randomUUID() + ".txt";
    const filePath = path.join("/var/data", filename);
    fs.writeFileSync(filePath, content);
    res.json({ success: true, filename });
}

export function appendFile(req: express.Request, res: express.Response) {
    const entry = req.body.entry;
    const logFile = "app-" + new Date().toISOString().slice(0, 10) + ".log";
    const filePath = path.join("/var/logs", logFile);
    fs.appendFileSync(filePath, entry + "\n");
    res.json({ success: true });
}
