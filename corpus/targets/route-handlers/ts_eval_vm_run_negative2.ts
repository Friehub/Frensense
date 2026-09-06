// SAFE: Used vm2 with a restricted sandbox that only exposes safe globals.

import { VM } from "vm2";

function runUserCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = { Math, JSON, Array, Object, String, Number, Boolean };
    const vm = new VM({
        timeout: 1000,
        sandbox,
        eval: false,
        wasm: false,
        fixAsync: false,
    });
    const output = vm.run(userCode);
    res.json({ output });
}

function executeFormula(req: Request, res: Response) {
    const formula = req.body.formula;
    const sandbox = { data: req.body.data, Math, JSON, Array };
    const vm = new VM({ timeout: 1000, sandbox, eval: false });
    const result = vm.run(`(${formula})`);
    res.json({ result });
}
