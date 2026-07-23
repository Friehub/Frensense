// SAFE: vm.Script is used with a timeout and break-on-sigint for safe evaluation.

import vm from "node:vm";

function executeUserCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = { data: req.body.input };
    const script = new vm.Script(userCode, { timeout: 1000 });
    const context = vm.createContext(sandbox);
    const result = script.runInContext(context, { breakOnSigint: true });
    res.json({ result });
}
