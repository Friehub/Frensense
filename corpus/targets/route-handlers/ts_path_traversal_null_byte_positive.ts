// [frensense]
// observation: The application terminates a path string at a null byte (%00) to bypass extension-based filters, allowing arbitrary file reads with a fake extension.
// impact: An attacker can supply "file.txt%00.html" — the null byte truncates the string at the application level, bypassing the ".html" extension check.
// improvement: Reject input containing null bytes; strip or validate before path operations.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import express from "express";
import fs from "fs";
import path from "path";

export function readHtmlFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    if (!filename.endsWith(".html")) {
        return res.status(403).send("Only HTML files allowed");
    }
    const filePath = path.join("/var/www", filename);
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function serveTemplate(req: express.Request, res: express.Response) {
    const template = req.query.template as string;
    if (!template.endsWith(".ejs")) {
        return res.status(403).send("Invalid template");
    }
    const content = fs.readFileSync(path.join("/var/templates", template));
    res.send(content);
}
