// SAFE: SVG content is sanitized to remove script tags and event handlers before serving
import express from "express";
import multer from "multer";
import { JSDOM } from "jsdom";
import fs from "fs";

const upload = multer({ dest: "uploads/" });

function sanitizeSvg(filePath: string): string {
    const svg = fs.readFileSync(filePath, "utf-8");
    const dom = new JSDOM(svg, { contentType: "image/svg+xml" });
    const doc = dom.window.document;
    const scripts = doc.querySelectorAll("script");
    scripts.forEach(s => s.remove());
    const elements = doc.querySelectorAll("*");
    elements.forEach(el => {
        const attrs = Array.from(el.attributes);
        for (const attr of attrs) {
            if (attr.name.startsWith("on")) {
                el.removeAttribute(attr.name);
            }
        }
    });
    return doc.documentElement.outerHTML;
}

export function uploadSVG(req: express.Request, res: express.Response) {
    const file = req.file!;
    const safeSvg = sanitizeSvg(file.path);
    res.setHeader("Content-Type", "image/svg+xml");
    res.send(safeSvg);
}
