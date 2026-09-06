// SAFE: File extension is verified after resolving the final path; null byte truncation cannot bypass because the full resolved path is checked
import express from "express";
import fs from "fs";
import path from "path";

const HTML_DIR = path.resolve("/var/www");
const TEMPLATE_DIR = path.resolve("/var/templates");

export function readHtmlFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    const safeName = path.basename(filename);
    const filePath = path.resolve(HTML_DIR, safeName);
    if (!filePath.startsWith(HTML_DIR)) {
        return res.status(403).send("Forbidden");
    }
    if (!filePath.endsWith(".html")) {
        return res.status(403).send("Only HTML files allowed");
    }
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function serveTemplate(req: express.Request, res: express.Response) {
    const template = req.query.template as string;
    const safeName = path.basename(template);
    const filePath = path.resolve(TEMPLATE_DIR, safeName);
    if (!filePath.startsWith(TEMPLATE_DIR) || !filePath.endsWith(".ejs")) {
        return res.status(403).send("Invalid template");
    }
    const content = fs.readFileSync(filePath);
    res.send(content);
}
