// [frensense]
// observation: An XML parser is configured to resolve external entities without restriction, allowing an attacker to include external URLs via DOCTYPE declarations.
// impact: SSRF via XML external entity (XXE) — the XML parser fetches the attacker-specified URL and includes the content in the parsed document, exposing internal resources.
// improvement: Disable external entity resolution in the XML parser configuration.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

import express from "express";
import { parseString } from "xml2js";

export function parseXmlData(req: express.Request, res: express.Response) {
    const xml = req.body.xml;
    parseString(xml, (err, result) => {
        if (err) return res.status(400).json({ error: "Invalid XML" });
        res.json(result);
    });
}

export function processXmlUpload(req: express.Request, res: express.Response) {
    const xmlData = req.files!.xmlFile.data.toString();
    parseString(xmlData, (err, result) => {
        if (err) return res.status(400).json({ error: "Invalid XML" });
        res.json(result);
    });
}
