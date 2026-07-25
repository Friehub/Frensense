// [frensense]
// observation: A user-controlled module path is passed to require() or import(), allowing the attacker to load arbitrary files using path traversal.
// impact: An attacker can require sensitive files (e.g., /etc/passwd, configuration files) and potentially execute code from attacker-controlled locations.
// improvement: Restrict require() to a pre-defined set of modules, or validate that the resolved path is within the allowed module directory.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import express from "express";

export function loadModule(req: express.Request, res: express.Response) {
    const modulePath = req.query.module as string;
    const mod = require(modulePath);
    res.json({ exports: Object.keys(mod) });
}

export function importPlugin(req: express.Request, res: express.Response) {
    const plugin = req.body.pluginPath;
    const mod = require(plugin);
    res.json(mod.run());
}
