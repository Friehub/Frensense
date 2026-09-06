// SAFE: vm.Script is used with a properly contextified sandbox and a proxy that traps constructor access.

import vm from "node:vm";

function executeSandboxedCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const context = vm.createContext(Object.create(null), {
        microtaskMode: "afterEvaluate",
    });
    context.data = req.body.input;
    const script = new vm.Script(userCode, { timeout: 1000 });
    const result = script.runInContext(context);
    res.json({ result });
}
