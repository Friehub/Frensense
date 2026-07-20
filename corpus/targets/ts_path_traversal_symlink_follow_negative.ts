// SAFE: Uses fs.realpathSync to resolve symlinks before checking the prefix, preventing symlink escape
import express from "express";
import fs from "fs";
import path from "path";

export function readFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/uploads", filename);
    const realPath = fs.realpathSync(filePath);
    if (!realPath.startsWith("/var/uploads")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(realPath, "utf-8");
    res.send(content);
}

export function getConfig(req: express.Request, res: express.Response) {
    const configFile = req.query.file as string;
    const fullPath = path.join("/var/config", configFile);
    const realPath = fs.realpathSync(fullPath);
    if (!realPath.startsWith("/var/config")) {
        return res.status(403).send("Forbidden");
    }
    const data = fs.readFileSync(realPath);
    res.send(data);
}
