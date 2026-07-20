// [frensense]
// observation: A file's original filename from user upload is reflected in the Content-Disposition header without encoding, allowing header injection or XSS if the browser interprets the filename as HTML.
// impact: An attacker can inject CRLF characters in the filename to inject headers, or inject quotes to break the filename attribute and trigger XSS in download dialogs.
// improvement: Sanitize or strip the filename before using it in Content-Disposition, or use a server-generated filename instead.

import express from "express";
import multer from "multer";

const upload = multer({ dest: "uploads/" });

export function downloadFile(req: express.Request, res: express.Response) {
    const file = req.file!;
    const filename = file.originalname;
    res.setHeader("Content-Disposition", `attachment; filename="${filename}"`);
    res.sendFile(file.path);
}

export function exportReport(req: express.Request, res: express.Response) {
    const reportName = req.body.reportName;
    res.setHeader("Content-Disposition", `attachment; filename="${reportName}.csv"`);
    res.send(csvData);
}
