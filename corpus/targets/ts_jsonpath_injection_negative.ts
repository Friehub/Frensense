// SAFE: Validated the JSONPath expression against an allowlist of permitted paths before evaluation.

import { JSONPath } from "jsonpath-plus";

const ALLOWED_PATHS = new Set([
    "$.user.name", "$.user.email", "$.user.role",
    "$.order.id", "$.order.total", "$.order.status",
    "$.items[*].name", "$.items[*].price",
]);

function queryData(req: Request, res: Response) {
    const path = req.body.path;
    if (!ALLOWED_PATHS.has(path)) {
        res.status(400).json({ error: "Path not allowed" });
        return;
    }
    const data = req.body.data || getDefaultData();
    const result = JSONPath({ path, json: data });
    res.json({ result });
}

function extractField(req: Request, res: Response) {
    const path = req.query.path as string;
    if (!ALLOWED_PATHS.has(path)) throw new Error("Path not allowed");
    const doc = getDocument(req.params.id);
    const result = JSONPath({ path, json: doc });
    res.json({ result });
}
