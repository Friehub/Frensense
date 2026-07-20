// [frensense]
// observation: User-controlled input is compiled as a Node.js module via Module._compile, allowing arbitrary code injection and execution.
// impact: An attacker can inject arbitrary JavaScript that runs as a loaded module, gaining access to Node.js internals and potentially the entire system.
// improvement: Avoid Module._compile entirely; use safer alternatives like the VM sandbox for isolated code evaluation.

import Module from "module";
import path from "path";

function executeUserModule(req: Request, res: Response) {
    const code = req.body.code;
    const filename = req.body.filename || "/tmp/user-module.js";
    const mod = new Module(filename, module.parent);
    mod.filename = filename;
    mod.paths = Module._nodeModulePaths(path.dirname(filename));
    mod._compile(code, filename);
    const result = mod.exports;
    res.json({ result });
}
