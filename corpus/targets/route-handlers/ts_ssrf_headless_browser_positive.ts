// [frensense]
// observation: A headless browser (Puppeteer/Playwright) is used to navigate to a user-controlled URL, allowing SSRF and arbitrary page rendering from attacker-chosen hosts.
// impact: The headless browser fetches internal pages, cloud metadata, or attacker-hosted content, potentially leaking data or executing JS from external sources.
// improvement: Validate the URL against an allowlist before calling goto(), or proxy through a URL validation service.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

import puppeteer from "puppeteer";
import express from "express";

export async function screenshotUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.goto(url, { waitUntil: "networkidle0" });
    const screenshot = await page.screenshot({ fullPage: true });
    await browser.close();
    res.setHeader("Content-Type", "image/png");
    res.send(screenshot);
}

export async function scrapeUrl(req: express.Request, res: express.Response) {
    const target = req.body.targetUrl;
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.goto(target, { waitUntil: "domcontentloaded" });
    const text = await page.evaluate(() => document.body.innerText);
    await browser.close();
    res.json({ text });
}
