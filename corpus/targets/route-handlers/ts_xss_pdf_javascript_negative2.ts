// SAFE: Uses a PDF library with JavaScript disabled explicitly; user content is rendered as safe text
import { PDFDocument, PDFTextField } from "pdf-lib";
import express from "express";

export async function generatePDF(req: express.Request, res: express.Response) {
    const userText = req.body.content;
    const doc = await PDFDocument.create();
    const page = doc.addPage([612, 792]);
    page.drawText(userText, { x: 100, y: 700, size: 12 });
    const pdfBytes = await doc.save();
    res.setHeader("Content-Type", "application/pdf");
    res.send(Buffer.from(pdfBytes));
}

export async function createInvoice(req: express.Request, res: express.Response) {
    const items = req.body.items;
    const doc = await PDFDocument.create();
    const page = doc.addPage([612, 792]);
    page.drawText("Invoice", { x: 100, y: 750, size: 16 });
    items.forEach((item: any, i: number) => {
        page.drawText(`${item.name}: $${item.price}`, { x: 100, y: 700 - i * 20, size: 12 });
    });
    const pdfBytes = await doc.save();
    res.setHeader("Content-Type", "application/pdf");
    res.send(Buffer.from(pdfBytes));
}
