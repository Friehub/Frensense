// SAFE: Null bytes are explicitly checked for and rejected before any file operation
import express from "express";
import fs from "fs";
import path from "path";

function isSafe(input: string): boolean {
    return !input.includes("\0");
}

export function readHtmlFile(req: express.Request, res: express.Response) {
    const filename = req.params.filename;
    if (!isSafe(filename)) {
        return res.status(400).send("Invalid filename");
    }
    if (!filename.endsWith(".html")) {
        return res.status(403).send("Only HTML files allowed");
    }
    const filePath = path.join("/var/www", path.basename(filename));
    const content = fs.readFileSync(filePath, "utf-8");
    res.send(content);
}

export function serveTemplate(req: express.Request, res: express.Response) {
    const template = req.query.template as string;
    if (!isSafe(template)) {
        return res.status(400).send("Invalid template");
    }
    if (!template.endsWith(".ejs")) {
        return res.status(403).send("Invalid template");
    }
    const content = fs.readFileSync(path.join("/var/templates", path.basename(template)));
    res.send(content);
}
