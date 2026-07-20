// SAFE: The filename is sanitized — only alphanumeric chars and safe punctuation are allowed
import express from "express";
import multer from "multer";

const upload = multer({ dest: "uploads/" });

function safeFilename(name: string): string {
    return name.replace(/[^a-zA-Z0-9._-]/g, "_");
}

export function downloadFile(req: express.Request, res: express.Response) {
    const file = req.file!;
    const filename = safeFilename(file.originalname);
    res.setHeader("Content-Disposition", `attachment; filename="${filename}"`);
    res.sendFile(file.path);
}

export function exportReport(req: express.Request, res: express.Response) {
    const reportName = safeFilename(req.body.reportName);
    res.setHeader("Content-Disposition", `attachment; filename="${reportName}.csv"`);
    res.send(csvData);
}
