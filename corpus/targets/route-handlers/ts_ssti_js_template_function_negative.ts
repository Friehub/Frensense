// SAFE: Replaced new Function() with an expression parser (mathjs) for safe evaluation of mathematical expressions.

import { evaluate } from "mathjs";

function computeExpression(req: Request, res: Response) {
    const expr = req.body.expression;
    try {
        const result = evaluate(expr);
        res.json({ result });
    } catch {
        res.status(400).json({ error: "Invalid expression" });
    }
}

function evaluateCondition(req: Request, res: Response) {
    const condition = req.query.rule as string;
    const ALLOWED_RULES: Record<string, (user: any, data: any) => boolean> = {
        "isAdmin": (u, d) => u.role === "admin",
        "isOwner": (u, d) => u.id === d.ownerId,
        "aboveThreshold": (u, d) => d.amount > 100,
    };
    const check = ALLOWED_RULES[condition];
    if (!check) throw new Error("Unknown rule");
    const passed = check(req.user, req.body);
    res.json({ passed });
}
