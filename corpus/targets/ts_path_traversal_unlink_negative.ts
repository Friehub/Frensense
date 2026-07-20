// SAFE: The filename is sanitized using path.basename, and the resolved path is verified to stay within the allowed directory
import express from "express";
import fs from "fs";
import path from "path";

const UPLOAD_DIR = path.resolve("/var/uploads");
const TEMP_DIR = path.resolve("/var/temp");

export function deleteFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const safeName = path.basename(filename);
    const filePath = path.resolve(UPLOAD_DIR, safeName);
    if (!filePath.startsWith(UPLOAD_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    if (!fs.existsSync(filePath)) {
        return res.status(404).json({ error: "File not found" });
    }
    fs.unlinkSync(filePath);
    res.json({ success: true });
}

export function removeDir(req: express.Request, res: express.Response) {
    const dir = req.query.dir as string;
    const safeName = path.basename(dir);
    const dirPath = path.resolve(TEMP_DIR, safeName);
    if (!dirPath.startsWith(TEMP_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    fs.rmSync(dirPath, { recursive: true, force: true });
    res.json({ success: true });
}
