// SAFE: PDF is generated from server-side HTML instead of fetching a user-controlled URL
import puppeteer from "puppeteer";
import express from "express";

export async function generatePDFFromHtml(req: express.Request, res: express.Response) {
    const htmlContent = req.body.html;
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.setContent(htmlContent, { waitUntil: "networkidle0" });
    const pdf = await page.pdf({ format: "A4" });
    await browser.close();
    res.setHeader("Content-Type", "application/pdf");
    res.send(pdf);
}
