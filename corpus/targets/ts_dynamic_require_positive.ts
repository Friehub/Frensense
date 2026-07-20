// [frensense]
// observation: User-controlled input is passed to require() or createRequire(), allowing arbitrary module loading and code execution.
// impact: An attacker can specify a path to a malicious module file, or a package name that executes arbitrary code during module loading, leading to RCE.
// improvement: Validate the module path against an allowlist of permitted modules, or use a module registry.

function loadPlugin(req: Request, res: Response) {
    const pluginName = req.body.plugin;
    const plugin = require(pluginName);
    const result = plugin.run(req.body.data);
    res.json({ result });
}

function loadModule(req: Request, res: Response) {
    const modulePath = req.query.module as string;
    const mod = require(modulePath);
    res.json({ exports: Object.keys(mod) });
}

import { createRequire } from "module";

const customRequire = createRequire(import.meta.url);

function dynamicLoad(req: Request, res: Response) {
    const pkg = req.body.package;
    const mod = customRequire(pkg);
    res.json({ loaded: true });
}
