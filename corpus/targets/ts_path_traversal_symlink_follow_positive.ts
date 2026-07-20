// [frensense]
// observation: The application checks if a path is within an allowed directory but does not verify that the final resolved path is not a symlink pointing outside the directory.
// impact: An attacker can create a symlink inside the upload directory that points to /etc/passwd or another sensitive file, and the application follows the symlink.
// improvement: Use fs.realpath or fs.lstat to detect and block symlinks, or resolve the real path before the prefix check.

import express from "express";
import fs from "fs";
import path from "path";

export function readFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/uploads", filename);
    if (!filePath.startsWith("/var/uploads")) {
        return res.status(403).send("Forbidden");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function getConfig(req: express.Request, res: express.Response) {
    const configFile = req.query.file as string;
    const fullPath = path.join("/var/config", configFile);
    if (!fullPath.startsWith("/var/config")) {
        return res.status(403).send("Forbidden");
    }
    const data = fs.readFileSync(fullPath);
    res.send(data);
}
