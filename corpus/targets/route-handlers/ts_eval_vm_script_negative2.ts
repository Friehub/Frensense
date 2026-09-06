// SAFE: Used vm2 with Script analogue and very limited sandbox to prevent sandbox escape.

import { VM, NodeVM } from "vm2";

function runScript(req: Request, res: Response) {
    const scriptCode = req.body.script;
    const vm = new VM({
        timeout: 1000,
        sandbox: { data: req.body.data, Math, JSON, Array, Object },
        eval: false,
        wasm: false,
    });
    vm.run(scriptCode);
    res.json({ executed: true });
}

function compileAndRun(req: Request, res: Response) {
    const code = req.query.code as string;
    const vm = new VM({
        timeout: 1000,
        sandbox: { Math, JSON },
        eval: false,
    });
    const output = vm.run(`String(${code})`);
    res.json({ output });
}
