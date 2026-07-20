// SAFE: Replaced new Function() with a safe expression parser (mathjs) for calculations and a rules engine for validation.

import { evaluate } from "mathjs";

function calculate(req: Request, res: Response) {
    const expression = req.body.expression;
    const data = req.body.data || {};
    const scope = { ...data, Math, JSON };
    const result = evaluate(expression, scope);
    res.json({ result });
}

function applyTransformation(req: Request, res: Response) {
    const results = req.body.items.map((item: any, index: number) => {
        return { ...item, index };
    });
    res.json(results);
}

function createValidator(req: Request, res: Response) {
    const VALIDATORS: Record<string, (v: any) => boolean> = {
        positive: v => typeof v === "number" && v > 0,
        nonEmpty: v => typeof v === "string" && v.length > 0,
        inRange: v => typeof v === "number" && v >= 0 && v <= 100,
    };
    const ruleName = req.body.rule;
    const validator = VALIDATORS[ruleName];
    if (!validator) throw new Error("Unknown rule");
    const isValid = req.body.values.every((v: any) => validator(v));
    res.json({ valid: isValid });
}
