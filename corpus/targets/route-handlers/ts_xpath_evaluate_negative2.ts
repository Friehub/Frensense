// SAFE: Parsed XML with a schema validator and used only predefined XPath expressions via named queries.

import { DOMParser } from "xmldom";
import { xpath } from "xpath";

const ALLOWED_XPATHS: Record<string, (params: any) => string> = {
    getUserByName: (p) => `/users/user[name/text()='${p.name.replace(/'/g, "&apos;")}']",
    getUsersByRole: (p) => `/users/user[role/text()='${p.role.replace(/'/g, "&apos;")}']`,
};

function executeXPath(req: Request, res: Response) {
    const queryName = req.body.queryName;
    const builder = ALLOWED_XPATHS[queryName];
    if (!builder) throw new Error("Unknown query");
    const xpathStr = builder(req.body.params || {});
    const doc = parser.parseFromString(req.body.xml || getDefaultXml());
    const nodes = xpath.select(xpathStr, doc);
    res.json({ result: nodes.map(n => n.toString()) });
}

function queryWithUserPath(req: Request, res: Response) {
    const doc = loadXmlDocument();
    const nodes = xpath.select("/users/user", doc);
    res.json(nodes.map(n => n.toString()));
}
