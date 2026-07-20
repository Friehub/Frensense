// [frensense]
// observation: vm.runInNewContext is called with a sandbox that does not freeze prototypes, allowing a sandbox escape via the constructor chain (e.g., this.constructor.constructor).
// impact: An attacker can access the global process object, require, and other Node.js internals, leading to full system compromise.
// improvement: Freeze the sandbox object and its prototypes, or use vm.Script with a properly contextified sandbox that removes access to constructors.

import vm from "node:vm";

function executeSandboxedCode(req: Request, res: Response) {
    const userCode = req.body.code;
    const sandbox = { data: req.body.input, console };
    const result = vm.runInNewContext(userCode, sandbox);
    res.json({ result });
}
