// SAFE: XML parser is configured to disable external entity resolution, preventing XXE-based SSRF
import express from "express";
import { parseString } from "xml2js";

export function parseXmlData(req: express.Request, res: express.Response) {
    const xml = req.body.xml;
    parseString(xml, {
        explicitChildren: true,
        preserveChildren: true,
        xml2js: {
            processEntities: false,
        },
        sax: {
            FEATURE: {
                "http://apache.org/xml/features/disallow-doctype-decl": true,
                "http://apache.org/xml/features/nonvalidating/load-external-dtd": false,
                "http://xml.org/sax/features/external-general-entities": false,
                "http://xml.org/sax/features/external-parameter-entities": false,
            },
        },
    }, (err, result) => {
        if (err) return res.status(400).json({ error: "Invalid XML" });
        res.json(result);
    });
}
