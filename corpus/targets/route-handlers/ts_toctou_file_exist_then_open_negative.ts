// SAFE: Opens the file directly without pre-checking existence; errors from open are handled instead
import express from "express";
import fs from "fs";
import path from "path";

export function readUserFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const filePath = path.join("/var/data", path.basename(filename));
    try {
        const content = fs.readFileSync(filePath, "utf-8");
        res.send(content);
    } catch (err: any) {
        if (err.code === "ENOENT") {
            return res.status(404).send("File not found");
        }
        throw err;
    }
}

export function deleteTempFile(req: express.Request, res: express.Response) {
    const filePath = req.body.path;
    try {
        fs.unlinkSync(filePath);
        res.json({ deleted: true });
    } catch (err: any) {
        if (err.code === "ENOENT") {
            return res.json({ deleted: false });
        }
        throw err;
    }
}
