// SAFE: Used path validation and a prefix allowlist to restrict import paths to a controlled directory.

import path from "path";

const ALLOWED_IMPORT_DIR = new Set([path.resolve("./features"), path.resolve("./plugins")]);

async function loadFeature(req: Request, res: Response) {
    const moduleName = req.body.module;
    const resolved = path.resolve("./features", moduleName);
    if (!ALLOWED_IMPORT_DIR.has(path.dirname(resolved))) {
        throw new Error("Import not allowed");
    }
    const mod = await import(resolved);
    const result = await mod.default(req.body.data);
    res.json({ result });
}

async function importPlugin(req: Request, res: Response) {
    const pluginPath = "./plugins/" + req.query.name;
    const resolved = path.resolve(pluginPath);
    if (!ALLOWED_IMPORT_DIR.has(path.dirname(resolved))) throw new Error("Invalid path");
    const plugin = await import(resolved);
    res.json({ loaded: true });
}
