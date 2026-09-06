// SAFE: Used a CEL expression that only allows predefined field names, validated before evaluation.

function restrictCelExpression(expr: string): string {
    const ALLOWED_FIELDS = new Set(["user", "data", "request", "now"]);
    const ALLOWED_ACCESSORS = new Set(["role", "status", "id", "amount", "ownerId", "name", "email"]);
    return expr.replace(/[a-zA-Z_][a-zA-Z0-9_]*/g, (match) => {
        if (ALLOWED_FIELDS.has(match) || ALLOWED_ACCESSORS.has(match) || /^\d+(\.\d+)?$/.test(match)) {
            return match;
        }
        return "null";
    });
}

function evaluateRule(req: Request, res: Response) {
    const expression = restrictCelExpression(req.body.expression);
    const context = { user: req.user, data: req.body.data, now: new Date() };
    const result = cel.execute(expression, context);
    res.json({ result });
}

function checkCondition(req: Request, res: Response) {
    const condition = restrictCelExpression(req.query.condition as string);
    const context = { request: req.body, user: req.user };
    const allowed = cel.execute(condition, context);
    res.json({ allowed: !!allowed });
}
