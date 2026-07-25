// [frensense]
// observation: User-controlled input is compiled as a vm.Script and run in a context, allowing arbitrary code execution via crafted scripts.
// impact: An attacker can craft input that compiles malicious JavaScript that runs in the VM, potentially escaping the sandbox to access host resources.
// improvement: Avoid running user-supplied scripts; use deterministic data transformations instead.
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

import vm from "vm";

function runScript(req: Request, res: Response) {
    const scriptCode = req.body.script;
    const sandbox = { result: null, data: req.body.data };
    const script = new vm.Script(scriptCode);
    const context = vm.createContext(sandbox);
    script.runInContext(context);
    res.json({ result: sandbox.result });
}

function compileAndRun(req: Request, res: Response) {
    const code = req.query.code as string;
    const sandbox = { output: "", console: { log: (msg: string) => { sandbox.output += msg; } } };
    const script = new vm.Script(`console.log(${code})`);
    const context = vm.createContext(sandbox);
    script.runInContext(context);
    res.json({ output: sandbox.output });
}
