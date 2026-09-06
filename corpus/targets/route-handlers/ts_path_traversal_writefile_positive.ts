// [frensense]
// observation: A user-controlled filename is concatenated with a base directory and passed to fs.writeFile or fs.appendFile without path traversal validation, allowing arbitrary file writes.
// impact: An attacker can overwrite critical system files, configuration files, or inject malicious scripts by traversing out of the upload directory.
// improvement: Validate and sanitize the path before writing; use path.basename to strip directory components.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import express from "express";
import fs from "fs";
import path from "path";

export function saveFile(req: express.Request, res: express.Response) {
    const filename = req.body.filename;
    const content = req.body.content;
    const filePath = "/var/data/" + filename;
    fs.writeFileSync(filePath, content);
    res.json({ success: true });
}

export function appendFile(req: express.Request, res: express.Response) {
    const logFile = req.query.file as string;
    const entry = req.body.entry;
    fs.appendFileSync(path.join("/var/logs", logFile), entry + "\n");
    res.json({ success: true });
}
