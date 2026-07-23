// SAFE: URL is validated against an allowlist before being passed to Puppeteer
import puppeteer from "puppeteer";
import express from "express";

const ALLOWED_DOMAINS = new Set(["docs.example.com", "report.example.com"]);

function isAllowed(url: string): boolean {
    try {
        const parsed = new URL(url);
        return parsed.protocol === "https:" && ALLOWED_DOMAINS.has(parsed.hostname);
    } catch {
        return false;
    }
}

export async function generatePDFFromUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    if (!isAllowed(url)) {
        return res.status(403).json({ error: "URL not allowed" });
    }
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.goto(url, { waitUntil: "networkidle0" });
    const pdf = await page.pdf({ format: "A4" });
    await browser.close();
    res.setHeader("Content-Type", "application/pdf");
    res.send(pdf);
}
