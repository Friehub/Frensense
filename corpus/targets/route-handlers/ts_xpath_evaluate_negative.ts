// SAFE: Replaced user-supplied XPath with predefined queries based on a limited set of allowed query names.

const PREDEFINED_QUERIES: Record<string, string> = {
    "users": "/users/user",
    "admins": "/users/user[role/text()='admin']",
    "active": "/users/user[status/text()='active']",
    "userById": "/users/user[@id=$id]",
};

import { XPathEvaluator } from "xpath";

const evaluator = new XPathEvaluator();

function executeXPath(req: Request, res: Response) {
    const queryName = req.body.queryName;
    const query = PREDEFINED_QUERIES[queryName];
    if (!query) throw new Error("Unknown query");
    const doc = parser.parseFromString(req.body.xml || getDefaultXml());
    const expr = evaluator.createExpression(query, [["id", "xs:string"]]);
    const result = expr.evaluate(doc, null, { id: req.body.id || "" });
    res.json({ result: result.stringValue });
}

function queryWithUserPath(req: Request, res: Response) {
    const doc = loadXmlDocument();
    const nodes = xpath.select("/users/user[name=$name]", doc, { name: req.query.name });
    res.json(nodes.map(n => n.toString()));
}
