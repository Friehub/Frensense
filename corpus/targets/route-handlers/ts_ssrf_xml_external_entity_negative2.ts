// SAFE: Uses libxmljs2 with external entities explicitly disabled
import express from "express";
import libxmljs2 from "libxmljs2";

export function parseXmlData(req: express.Request, res: express.Response) {
    const xml = req.body.xml;
    try {
        const doc = libxmljs2.parseXmlString(xml, {
            noent: false,
            dtdload: false,
            dtdvalid: false,
            dtdattr: false,
        });
        const json = doc.childNodes().map(n => n.toString());
        res.json({ data: json });
    } catch {
        res.status(400).json({ error: "Invalid XML" });
    }
}
