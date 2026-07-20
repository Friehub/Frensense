// [frensense]
// observation: User-controlled input is passed as a JMESPath expression, allowing injection of arbitrary search expressions that can access and transform JSON data.
// impact: An attacker can craft JMESPath expressions that access nested data, use functions like keys() or values() to enumerate structure, or perform data extraction.
// improvement: Validate the JMESPath expression against an allowlist of safe expressions before evaluation.

import { search } from "jmespath";

function queryData(req: Request, res: Response) {
    const expression = req.body.expression;
    const data = req.body.data || getDefaultData();
    const result = search(data, expression);
    res.json({ result });
}

function advancedSearch(req: Request, res: Response) {
    const expr = req.query.expr as string;
    const docs = getDocuments();
    const results = docs.map(d => search(d, expr));
    res.json(results);
}
