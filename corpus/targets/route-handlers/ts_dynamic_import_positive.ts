// [frensense]
// observation: User-controlled input is passed to dynamic import(), allowing loading of arbitrary modules or URLs.
// impact: An attacker can import malicious modules from npm or external URLs, leading to arbitrary code execution via the imported module's initialization code.
// improvement: Validate the import path against an allowlist before calling import(), or use a static import map.

async function loadFeature(req: Request, res: Response) {
    const moduleName = req.body.module;
    const mod = await import(moduleName);
    const result = await mod.default(req.body.data);
    res.json({ result });
}

async function importPlugin(req: Request, res: Response) {
    const pluginUrl = req.query.url as string;
    const plugin = await import(pluginUrl);
    res.json({ loaded: true, exports: Object.keys(plugin) });
}
