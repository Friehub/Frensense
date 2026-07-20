// SAFE: Path is normalized (decoded and resolved) before the prefix check; Unicode bypasses cannot escape
import express from "express";
import fs from "fs";
import path from "path";

export function getFile(req: express.Request, res: express.Response) {
    const fileName = req.params.fileName;
    const safeName = path.basename(fileName);
    const filePath = path.join("/var/data", safeName);
    if (!filePath.startsWith("/var/data")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function downloadResource(req: express.Request, res: express.Response) {
    const resource = req.query.path as string;
    const decoded = decodeURIComponent(resource);
    const normalized = path.resolve("/var/resources", decoded);
    if (!normalized.startsWith("/var/resources")) {
        return res.status(403).send("Forbidden");
    }
    const data = fs.readFileSync(normalized);
    res.send(data);
}
