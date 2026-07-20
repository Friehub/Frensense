// SAFE: A random server-generated filename is used instead of the user-provided original name
import express from "express";
import multer from "multer";
import crypto from "crypto";
import path from "path";

const upload = multer({ dest: "uploads/" });

export function downloadFile(req: express.Request, res: express.Response) {
    const file = req.file!;
    const ext = path.extname(file.originalname);
    const safeName = crypto.randomUUID() + ext;
    res.setHeader("Content-Disposition", `attachment; filename="${safeName}"`);
    res.sendFile(file.path);
}

export function exportReport(req: express.Request, res: express.Response) {
    const safeName = "report-" + crypto.randomUUID() + ".csv";
    res.setHeader("Content-Disposition", `attachment; filename="${safeName}"`);
    res.send(csvData);
}
