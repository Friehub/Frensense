// SAFE: Used a sandboxed require via vm2 module resolution with path allowlisting.

import { NodeVM } from "vm2";
import path from "path";

const ALLOWED_PLUGIN_DIR = path.resolve("./plugins");

function loadPlugin(req: Request, res: Response) {
    const pluginName = req.body.plugin;
    const resolved = path.resolve(ALLOWED_PLUGIN_DIR, pluginName);
    if (!resolved.startsWith(ALLOWED_PLUGIN_DIR)) throw new Error("Invalid path");
    const vm = new NodeVM({
        require: {
            external: true,
            root: [ALLOWED_PLUGIN_DIR],
        },
    });
    const plugin = vm.run(`module.exports = require("${resolved}")`);
    const result = plugin.run(req.body.data);
    res.json({ result });
}
