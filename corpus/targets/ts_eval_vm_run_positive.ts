// [frensense]
// observation: User-controlled input is passed to vm.runInNewContext() or vm.runInThisContext(), allowing sandbox escape and arbitrary code execution.
// impact: An attacker can craft input that escapes the VM sandbox and gains access to the host system's globals, enabling RCE and full server compromise.
// improvement: Avoid user code evaluation entirely, or use a properly sandboxed environment with minimal globals.

import vm from "vm";

function runUserCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = { output: "", console: { log: (...args: any[]) => { sandbox.output += args.join(" "); } } };
    vm.runInNewContext(userCode, sandbox);
    res.json({ output: sandbox.output });
}

function executeFormula(req: Request, res: Response) {
    const formula = req.body.formula;
    const sandbox = { data: req.body.data, result: null };
    vm.runInNewContext(`result = ${formula}`, sandbox);
    res.json({ result: sandbox.result });
}

function runInGlobalContext(req: Request, res: Response) {
    const code = req.query.code as string;
    vm.runInThisContext(code);
    res.json({ executed: true });
}
