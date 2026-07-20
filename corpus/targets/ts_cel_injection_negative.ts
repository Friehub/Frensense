// SAFE: Replaced user-supplied CEL expressions with predefined rule names mapped to safe expressions.

const RULE_REGISTRY: Record<string, string> = {
    "isAdmin": "user.role == 'admin'",
    "isActive": "user.status == 'active'",
    "ownData": "user.id == data.ownerId",
    "aboveThreshold": "data.amount > 100.0",
    "withinBusinessHours": "now.hours >= 9 && now.hours < 17",
};

function evaluateRule(req: Request, res: Response) {
    const ruleName = req.body.rule;
    const expression = RULE_REGISTRY[ruleName];
    if (!expression) throw new Error("Unknown rule");
    const context = { user: req.user, data: req.body.data, now: new Date() };
    const result = cel.execute(expression, context);
    res.json({ result });
}

function checkCondition(req: Request, res: Response) {
    const ruleName = req.query.rule as string;
    const expression = RULE_REGISTRY[ruleName];
    if (!expression) throw new Error("Unknown condition");
    const context = { request: req.body, user: req.user };
    const allowed = cel.execute(expression, context);
    res.json({ allowed: !!allowed });
}
