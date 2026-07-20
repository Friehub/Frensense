// SAFE: Used vm2 sandbox to limit available globals and prevent escape to the host system.

import { VM } from "vm2";

function calculate(req: Request, res: Response) {
    const expression = req.body.expression;
    const data = req.body.data || {};
    const vm = new VM({
        timeout: 1000,
        sandbox: { data, Math, JSON, parseInt, parseFloat, Array, Object },
    });
    const result = vm.run(`(function() { return (${expression}); })()`);
    res.json({ result });
}

function applyTransformation(req: Request, res: Response) {
    const transformCode = req.body.transformer;
    const vm = new VM({
        timeout: 500,
        sandbox: { Math, JSON, Array, Object },
    });
    const results = req.body.items.map((item: any, index: number) => {
        return vm.run(`(function(item, index) { return (${transformCode}); })(${JSON.stringify(item)}, ${index})`);
    });
    res.json(results);
}

function createValidator(req: Request, res: Response) {
    const ruleCode = req.body.rule;
    const vm = new VM({ timeout: 500, sandbox: { Number, String, Array, Math } });
    const validator = vm.run(`(function(value) { return !!(${ruleCode}); })`);
    const isValid = req.body.values.every((v: any) => validator(v));
    res.json({ valid: isValid });
}
