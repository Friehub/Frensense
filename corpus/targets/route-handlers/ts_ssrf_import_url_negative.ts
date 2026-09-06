// SAFE: Dynamic import is restricted to a pre-defined set of local plugin modules only
import express from "express";

const ALLOWED_PLUGINS = new Set(["./plugins/analytics", "./plugins/logging", "./plugins/export"]);

export async function loadPlugin(req: express.Request, res: express.Response) {
    const pluginName = req.query.plugin as string;
    if (!ALLOWED_PLUGINS.has(pluginName)) {
        return res.status(403).json({ error: "Plugin not allowed" });
    }
    const plugin = await import(pluginName);
    const result = plugin.default(req, res);
    res.json({ result });
}

export async function executeScript(req: express.Request, res: express.Response) {
    return res.status(403).json({ error: "Dynamic scripts not supported" });
}
