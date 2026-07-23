// SAFE: Replaced dynamic require() with a plugin registry that maps plugin names to fixed module imports.

import PluginA from "./plugins/plugin-a";
import PluginB from "./plugins/plugin-b";
import PluginC from "./plugins/plugin-c";

const PLUGIN_REGISTRY: Record<string, { run: (data: any) => any }> = {
    "plugin-a": PluginA,
    "plugin-b": PluginB,
    "plugin-c": PluginC,
};

function loadPlugin(req: Request, res: Response) {
    const pluginName = req.body.plugin;
    const plugin = PLUGIN_REGISTRY[pluginName];
    if (!plugin) throw new Error("Unknown plugin");
    const result = plugin.run(req.body.data);
    res.json({ result });
}

function loadModule(req: Request, res: Response) {
    const allowedModules: Record<string, object> = {
        "lodash": require("lodash"),
        "moment": require("moment"),
        "uuid": require("uuid"),
    };
    const moduleName = req.query.module as string;
    const mod = allowedModules[moduleName];
    if (!mod) throw new Error("Module not allowed");
    res.json({ exports: Object.keys(mod) });
}
