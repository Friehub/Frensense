// SAFE: Path is normalized using path.resolve, then verified to remain within the allowed base directory
import express from "express";
import fs from "fs";
import path from "path";

const BASE_DIR = path.resolve("/var/data");

function isPathSafe(input: string): boolean {
    const decoded = decodeURIComponent(input);
    const resolved = path.resolve(BASE_DIR, decoded);
    return resolved.startsWith(BASE_DIR);
}

export function getFile(req: express.Request, res: express.Response) {
    const fileName = req.params.fileName;
    if (!isPathSafe(fileName)) {
        return res.status(403).send("Forbidden");
    }
    const filePath = path.resolve(BASE_DIR, fileName);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function downloadResource(req: express.Request, res: express.Response) {
    const resource = req.query.path as string;
    if (!isPathSafe(resource)) {
        return res.status(403).send("Forbidden");
    }
    const fullPath = path.resolve(BASE_DIR, resource);
    const data = fs.readFileSync(fullPath);
    res.send(data);
}
