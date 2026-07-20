// [frensense]
// observation: A PDF is generated from user input and served inline, allowing injection of JavaScript actions via PDF form fields or annotations.
// impact: An attacker can embed JavaScript in a PDF file that executes in the PDF viewer's JavaScript engine, typically within the origin's context.
// improvement: Disable JavaScript in PDF generation, or sanitize user input before embedding in PDFs.

import pdfkit from "pdfkit";
import express from "express";

export function generatePDF(req: express.Request, res: express.Response) {
    const userText = req.body.content;
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
    items.forEach((item: any) => doc.text(`${item.name}: $${item.price}`));
    doc.end();
}
