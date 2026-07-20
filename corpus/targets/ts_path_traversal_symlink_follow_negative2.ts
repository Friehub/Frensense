// SAFE: Uses lstat to verify the file is not a symlink before reading; also checks real path prefix
import express from "express";
import fs from "fs";
import path from "path";

export function readFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/uploads", path.basename(filename));
    const stat = fs.lstatSync(filePath);
    if (stat.isSymbolicLink()) {
        return res.status(403).send("Symlinks not allowed");
    }
    const realPath = fs.realpathSync(filePath);
    if (!realPath.startsWith("/var/uploads")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(realPath, "utf-8");
    res.send(content);
}

export function getConfig(req: express.Request, res: express.Response) {
    const configFile = req.query.file as string;
    const fullPath = path.join("/var/config", path.basename(configFile));
    const stat = fs.lstatSync(fullPath);
    if (stat.isSymbolicLink()) {
        return res.status(403).send("Symlinks not allowed");
    }
    const realPath = fs.realpathSync(fullPath);
    if (!realPath.startsWith("/var/config")) {
        return res.status(403).send("Forbidden");
    }
    const data = fs.readFileSync(realPath);
    res.send(data);
}
