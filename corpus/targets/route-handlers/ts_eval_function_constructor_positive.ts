// [frensense]
// observation: User-controlled input is passed to the Function() constructor and invoked, allowing arbitrary code execution on the server.
// impact: An attacker can inject arbitrary JavaScript code that executes with the server's privileges, leading to full application compromise.
// improvement: Never use new Function() with user input; use a sandboxed evaluator or a safe expression parser.

function calculate(req: Request, res: Response) {
    const expression = req.body.expression;
    const fn = new Function("data", "return " + expression);
    const result = fn(req.body.data);
    res.json({ result });
}

function applyTransformation(req: Request, res: Response) {
    const transformCode = req.body.transformer;
    const transformer = new Function("item", "index", transformCode);
    const results = req.body.items.map((item: any, index: number) => transformer(item, index));
    res.json(results);
}

function createValidator(req: Request, res: Response) {
    const ruleCode = req.body.rule;
    const validator = Function("value", "return " + ruleCode);
    const isValid = req.body.values.every((v: any) => validator(v));
    res.json({ valid: isValid });
}
