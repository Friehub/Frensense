// [frensense]
// observation: An SVG file uploaded by a user is served with Content-Type: image/svg+xml and rendered inline in the browser, allowing the SVG's <script> tags to execute.
// impact: An attacker uploads an SVG containing embedded JavaScript; when other users view the image, the script executes in the context of the application.
// improvement: Serve uploaded SVG files with Content-Disposition: attachment to prevent inline rendering, or strip script tags from SVGs.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";
import multer from "multer";

const upload = multer({ dest: "uploads/" });

export function uploadSVG(req: express.Request, res: express.Response) {
    const file = req.file!;
    res.setHeader("Content-Type", "image/svg+xml");
    res.sendFile(file.path);
}

export async function serveUploadedFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    res.setHeader("Content-Type", "image/svg+xml");
    res.sendFile(`uploads/${filename}`);
}
