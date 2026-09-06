// SAFE: PDF generation uses a library that does not support JavaScript; user text is rendered as plain text in the PDF
import pdfkit from "pdfkit";
import express from "express";

function stripJs(str: string): string {
    return str.replace(/<script[\s\S]*?<\/script>/gi, "").replace(/javascript:/gi, "blocked:");
}

export function generatePDF(req: express.Request, res: express.Response) {
    const userText = stripJs(req.body.content);
    const doc = new pdfkit();
    res.setHeader("Content-Type", "application/pdf");
    doc.pipe(res);
    doc.fontSize(12).text(userText, 100, 100);
    doc.end();
}

export function createInvoice(req: express.Request, res: express.Response) {
    const items = req.body.items;
    const doc = new pdfkit();
    res.setHeader("Content-Type", "application/pdf");
    doc.pipe(res);
    doc.fontSize(16).text("Invoice");
    items.forEach((item: any) => doc.text(`${stripJs(item.name)}: $${item.price}`));
    doc.end();
}
