// [frensense]
// observation: User-controlled input is passed as a JSONPath expression, allowing injection of arbitrary path expressions that can access any part of the JSON document.
// impact: An attacker can craft JSONPath expressions that iterate over all nodes, use filters to extract sensitive data, or trigger ReDoS via complex path patterns.
// improvement: Validate the JSONPath expression against an allowlist of permitted paths before evaluation.
// cwe: CWE-643
// cvss: 7.5
// owasp: A03:2021
// severity: High

import { JSONPath } from "jsonpath-plus";

function queryData(req: Request, res: Response) {
    const path = req.body.path;
    const data = req.body.data || getDefaultData();
    const result = JSONPath({ path, json: data });
    res.json({ result });
}

function extractField(req: Request, res: Response) {
    const path = req.query.path as string;
    const doc = getDocument(req.params.id);
    const result = JSONPath({ path, json: doc });
    res.json({ result });
}
