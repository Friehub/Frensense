// [frensense]
// observation: User-uploaded HTML files are served with Content-Type: text/html, causing the browser to render them as web pages with full script execution capability.
// impact: An attacker uploads a .html file containing phishing forms or malicious JavaScript; when accessed, the browser executes it in the origin context, enabling cookie theft or credential harvesting.
// improvement: Serve uploaded HTML files with Content-Type: text/plain or application/octet-stream to prevent rendering as HTML.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";
import multer from "multer";

const upload = multer({ dest: "uploads/" });

export function uploadHTML(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.setHeader("Content-Type", "text/html");
    res.sendFile(file.path);
}

export function serveFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    res.setHeader("Content-Type", "text/html");
    res.sendFile(`uploads/${filename}`);
}
