// SAFE: Browser uses a proxy that blocks private IP ranges; navigation to blocked hosts fails at network level
import puppeteer from "puppeteer";
import express from "express";

export async function screenshotUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    const browser = await puppeteer.launch({
        args: ["--proxy-server=internal-proxy:8080"],
    });
    const page = await browser.newPage();
    await page.authenticate({ username: "proxy-user", password: "proxy-pass" });
    try {
        await page.goto(url, { waitUntil: "networkidle0", timeout: 10000 });
    } catch {
        await browser.close();
        return res.status(502).json({ error: "Failed to load URL" });
    }
    const screenshot = await page.screenshot({ fullPage: true });
    await browser.close();
    res.setHeader("Content-Type", "image/png");
    res.send(screenshot);
}
