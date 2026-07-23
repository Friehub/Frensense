// [frensense]
// observation: A PDF generation library (wkhtmltopdf, puppeteer) is invoked with a user-controlled URL, allowing the PDF renderer to make HTTP requests to internal services.
// impact: SSRF via PDF generator — the headless browser fetches internal pages and embeds them in the PDF, exposing internal dashboards, metadata, or admin panels.
// improvement: Validate the URL against an allowlist before passing it to the PDF generator; or use a URL sanitizer that blocks private IPs.

import puppeteer from "puppeteer";
import express from "express";

export async function generatePDFFromUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.goto(url, { waitUntil: "networkidle0" });
    const pdf = await page.pdf({ format: "A4" });
    await browser.close();
    res.setHeader("Content-Type", "application/pdf");
    res.send(pdf);
}
