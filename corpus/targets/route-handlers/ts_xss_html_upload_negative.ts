// SAFE: Uploaded HTML files are served with Content-Type: text/plain or octet-stream, preventing browser execution
import express from "express";
import multer from "multer";

const upload = multer({ dest: "uploads/" });

const TEXT_TYPES = new Set([".txt", ".md", ".csv"]);

export function uploadHTML(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.setHeader("Content-Type", "text/plain");
    res.setHeader("Content-Disposition", "attachment");
    res.sendFile(file.path);
}

export function serveFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    res.setHeader("Content-Type", "application/octet-stream");
    res.setHeader("Content-Disposition", "attachment");
    res.sendFile(`uploads/${filename}`);
}
