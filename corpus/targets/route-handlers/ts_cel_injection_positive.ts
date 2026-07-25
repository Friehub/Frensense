// [frensense]
// observation: User-controlled input is passed as a Common Expression Language (CEL) expression, allowing injection of arbitrary expressions that access data fields.
// impact: An attacker can craft CEL expressions that access all fields of the input object, iterate over maps, or use CEL functions to extract sensitive data.
// improvement: Validate the CEL expression against an allowlist or use a fixed expression with variables for user input.
// cwe: CWE-94
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

import { cel } from "cel-js";

function evaluateRule(req: Request, res: Response) {
    const expression = req.body.expression;
    const context = { user: req.user, data: req.body.data, now: new Date() };
    const result = cel.execute(expression, context);
    res.json({ result });
}

function checkCondition(req: Request, res: Response) {
    const condition = req.query.condition as string;
    const context = { request: req.body, user: req.user };
    const allowed = cel.execute(condition, context);
    if (allowed) {
        res.json({ allowed: true });
    } else {
        res.status(403).json({ error: "Access denied" });
    }
}
