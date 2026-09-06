// SAFE: Replaced dynamic import() with a static import map, loading only pre-vetted modules.

import FeatureA from "./features/feature-a";
import FeatureB from "./features/feature-b";
import FeatureC from "./features/feature-c";

const FEATURE_MAP: Record<string, { default: (data: any) => any }> = {
    "feature-a": FeatureA,
    "feature-b": FeatureB,
    "feature-c": FeatureC,
};

async function loadFeature(req: Request, res: Response) {
    const moduleName = req.body.module;
    const feature = FEATURE_MAP[moduleName];
    if (!feature) throw new Error("Unknown feature");
    const result = await feature.default(req.body.data);
    res.json({ result });
}

async function importPlugin(req: Request, res: Response) {
    const allowedPlugins: Record<string, object> = {
        "lodash": await import("lodash"),
        "moment": await import("moment"),
    };
    const pluginName = req.query.name as string;
    const plugin = allowedPlugins[pluginName];
    if (!plugin) throw new Error("Plugin not allowed");
    res.json({ loaded: true, exports: Object.keys(plugin) });
}
