// [frensense]
// observation: User-controlled input is passed to new Function() or eval(), allowing arbitrary JavaScript code execution through crafted input.
// impact: An attacker can inject arbitrary JavaScript code that executes in the server process, leading to complete server compromise, data exfiltration, or lateral movement.
// improvement: Avoid new Function() and eval() with user input; use safer alternatives like JSON.parse or predefined logic maps.
// cwe: CWE-1336
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

function computeExpression(req: Request, res: Response) {
    const expr = req.body.expression;
    const fn = new Function("return " + expr);
    const result = fn();
    res.json({ result });
}

function evaluateCondition(req: Request, res: Response) {
    const condition = req.query.rule as string;
    const check = new Function("user", "data", "return " + condition);
    const passed = check(req.user, req.body);
    res.json({ passed });
}
