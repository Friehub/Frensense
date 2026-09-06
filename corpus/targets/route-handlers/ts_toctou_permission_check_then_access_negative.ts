// SAFE: Opens the file first, then checks permissions on the open file descriptor using fstat (immune to file swaps)
import express from "express";
import fs from "fs";
import path from "path";

export function readUserLog(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/logs/users", path.basename(filename));
    const fd = fs.openSync(filePath, "r");
    try {
        const stat = fs.fstatSync(fd);
        if (stat.uid !== req.session.userId) {
            return res.status(403).send("Not your log file");
        }
        const content = fs.readFileSync(fd, "utf-8");
        res.send(content);
    } catch (err: any) {
        if (err.code === "ENOENT") return res.status(404).send("Not found");
        throw err;
    } finally {
        fs.closeSync(fd);
    }
}

export function deleteFile(req: express.Request, res: express.Response) {
    const filePath = req.body.filePath;
    try {
        fs.unlinkSync(filePath);
        res.json({ success: true });
    } catch (err: any) {
        if (err.code === "ENOENT") return res.status(404).send("Not found");
        throw err;
    }
}
