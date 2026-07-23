// SAFE: Uses open + read in a single operation with file locking to prevent TOCTOU
import express from "express";
import fs from "fs";
import path from "path";

export function readUserFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/data", path.basename(filename));
    const fd = fs.openSync(filePath, "r");
    try {
        fs.flockSync(fd, "sh");
        const content = fs.readFileSync(fd, "utf-8");
        res.send(content);
    } catch (err: any) {
        if (err.code === "ENOENT") {
            return res.status(404).send("File not found");
        }
        throw err;
    } finally {
        fs.flockSync(fd, "un");
        fs.closeSync(fd);
    }
}
