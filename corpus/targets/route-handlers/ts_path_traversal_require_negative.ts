// SAFE: Module paths are restricted to a pre-defined allowlist of known modules
import express from "express";

const ALLOWED_MODULES = new Set([
    "./plugins/analytics",
    "./plugins/logging",
    "./plugins/export",
    "./utils/helpers",
]);

export function loadModule(req: express.Request, res: express.Response) {
    const modulePath = req.query.module as string;
    if (!ALLOWED_MODULES.has(modulePath)) {
        return res.status(403).json({ error: "Module not allowed" });
    }
    const mod = require(modulePath);
    res.json({ exports: Object.keys(mod) });
}

export function importPlugin(req: express.Request, res: express.Response) {
    return res.status(403).json({ error: "Dynamic plugin import disabled" });
}
