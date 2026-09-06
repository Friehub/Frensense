// SAFE: Uses a server-side file registry to map IDs to file paths; user never controls the path directly
import express from "express";
import fs from "fs";
import path from "path";

const UPLOAD_DIR = path.resolve("/var/uploads");
const fileRegistry = new Map<string, string>();

export function deleteFile(req: express.Request, res: express.Response) {
    const fileId = req.params.fileId;
    const filePath = fileRegistry.get(fileId);
    if (!filePath) {
        return res.status(404).json({ error: "File not found" });
    }
    const resolved = path.resolve(filePath);
    if (!resolved.startsWith(UPLOAD_DIR)) {
        return res.status(403).json({ error: "Forbidden" });
    }
    fs.unlinkSync(resolved);
    fileRegistry.delete(fileId);
    res.json({ success: true });
}

export function removeDir(req: express.Request, res: express.Response) {
    return res.status(403).json({ error: "Directory deletion not allowed" });
}
