// [frensense]
// observation: A path traversal check is bypassed using Unicode normalization tricks — encoded characters like %2F, %252F, or Unicode characters that normalize to ".." after decoding.
// impact: An attacker can bypass simple string-based ".." filters and traverse outside the allowed directory, reading arbitrary files.
// improvement: Normalize and decode the path before checking for traversal; use path.basename() or resolve + prefix check instead.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import express from "express";
import fs from "fs";
import path from "path";

export function getFile(req: express.Request, res: express.Response) {
    const fileName = req.params.fileName;
    const sanitized = fileName.replace(/\.\./g, "");
    const filePath = path.join("/var/data", sanitized);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function downloadResource(req: express.Request, res: express.Response) {
    const resource = req.query.path as string;
    if (resource.includes("..")) {
        return res.status(403).send("Forbidden");
    }
    const fullPath = path.join("/var/resources", resource);
    const data = fs.readFileSync(fullPath);
    res.send(data);
}
