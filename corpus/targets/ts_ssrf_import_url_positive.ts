// [frensense]
// observation: The application uses dynamic import() with a user-supplied URL or module specifier, allowing the attacker to load arbitrary remote modules.
// impact: SSRF via import() — the server fetches a remote module from an attacker-controlled URL, potentially executing malicious code if the module is evaluated.
// improvement: Restrict import() to a pre-defined allowlist of modules, or validate that the specifier does not point to remote URLs.

import express from "express";

export async function loadPlugin(req: express.Request, res: express.Response) {
    const pluginUrl = req.query.plugin as string;
    const plugin = await import(pluginUrl);
    const result = plugin.default(req, res);
    res.json({ result });
}

export async function executeScript(req: express.Request, res: express.Response) {
    const scriptUrl = req.body.scriptUrl;
    const mod = await import(scriptUrl);
    await mod.run(req.body.args);
    res.json({ success: true });
}
