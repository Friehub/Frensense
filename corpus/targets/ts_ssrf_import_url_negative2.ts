// SAFE: Import URLs are validated to ensure they are local file paths, not remote URLs
import express from "express";
import path from "path";

const PLUGIN_DIR = path.resolve("./plugins");

export async function loadPlugin(req: express.Request, res: express.Response) {
    const pluginName = req.query.plugin as string;
    const safeName = pluginName.replace(/[^a-zA-Z0-9_-]/g, "");
    const pluginPath = path.join(PLUGIN_DIR, safeName);
    if (!pluginPath.startsWith(PLUGIN_DIR)) {
        return res.status(403).json({ error: "Invalid plugin path" });
    }
    try {
        const plugin = await import(pluginPath);
        const result = plugin.default(req, res);
        res.json({ result });
    } catch {
        res.status(500).json({ error: "Plugin load failed" });
    }
}
