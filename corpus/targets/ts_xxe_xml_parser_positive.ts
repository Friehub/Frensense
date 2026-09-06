// [frensense]
// observation: XML is parsed using DOMParser without disabling external entity resolution, making the application vulnerable to XML External Entity (XXE) injection.
// impact: An attacker can supply an XML payload that reads local files, performs SSRF, or causes denial of service via entity expansion.
// improvement: Disable external entity resolution and DTD processing when parsing XML. Set resolveEntities: false or equivalent parser option.
// cwe: CWE-611
// cvss: 8.5
// owasp: A05:2021

import express from "express";
import { Router } from "express";

const router = Router();

router.post("/parse", (req: express.Request, res: express.Response) => {
    const parser = new DOMParser();
    const doc = parser.parseFromString(req.body.xml as string, "text/xml");
    const title = doc.querySelector("title")?.textContent || "";
    res.send(title);
});