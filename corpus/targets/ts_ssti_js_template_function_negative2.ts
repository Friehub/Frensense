// SAFE: Used a constrained sandbox with vm2 or safe-eval that limits the available globals for code evaluation.

import { VM } from "vm2";

function computeExpression(req: Request, res: Response) {
    const expr = req.body.expression;
    const vm = new VM({
        timeout: 1000,
        sandbox: { Math, JSON, parseInt, parseFloat },
    });
    const result = vm.run(`(${expr})`);
    res.json({ result });
}

function evaluateCondition(req: Request, res: Response) {
    const condition = req.query.rule as string;
    const vm = new VM({
        timeout: 500,
        sandbox: {
            user: req.user,
            data: req.body,
            Date, Math, JSON,
        },
    });
    const result = vm.run(`!!(${condition})`);
    res.json({ passed: result });
}
