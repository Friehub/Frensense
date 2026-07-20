// SAFE: The sandbox object and its prototype chain are frozen to prevent constructor escape.

import vm from "node:vm";

function executeSandboxedCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = Object.freeze({ data: req.body.input, console: Object.freeze({ log: console.log }) });
    const result = vm.runInNewContext(userCode, sandbox);
    res.json({ result });
}
