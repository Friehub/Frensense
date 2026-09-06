// SAFE: Module path is resolved and checked against the allowed plugin directory before requiring
import express from "express";
import path from "path";

const PLUGIN_DIR = path.resolve("./plugins");

export function loadModule(req: express.Request, res: express.Response) {
    const modulePath = req.query.module as string;
    const safeName = path.basename(modulePath).replace(/[^a-zA-Z0-9_-]/g, "");
    const resolved = path.resolve(PLUGIN_DIR, safeName);
    if (!resolved.startsWith(PLUGIN_DIR)) {
        return res.status(403).json({ error: "Invalid module" });
    }
    try {
        const mod = require(resolved);
        res.json({ exports: Object.keys(mod) });
    } catch {
        res.status(404).json({ error: "Module not found" });
    }
}
