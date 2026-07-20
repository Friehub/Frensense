// [frensense]
// observation: The full XPath expression is taken directly from user input and evaluated against the XML document, allowing arbitrary XPath injection.
// impact: An attacker can craft arbitrary XPath expressions to extract any data from the XML document, including out-of-band data exfiltration via XPath functions.
// improvement: Never evaluate arbitrary XPath expressions from user input; use predefined queries with parameter binding instead.

import { DOMParser } from "xmldom";
import { xpath } from "xpath";

const parser = new DOMParser();

function executeXPath(req: Request, res: Response) {
    const userXPath = req.body.expression;
    const xmlString = req.body.xml || getDefaultXml();
    const doc = parser.parseFromString(xmlString);
    const result = xpath.evaluate(userXPath, doc, null, XPathResult.ANY_TYPE, null);
    res.json({ result: result.stringValue });
}

function queryWithUserPath(req: Request, res: Response) {
    const expr = req.query.xpath as string;
    const doc = loadXmlDocument();
    const nodes = xpath.select(expr, doc);
    res.json(nodes.map(n => n.toString()));
}
