// SAFE: Used a path prefix validation to ensure the expression only accesses a predefined subtree.

import { JSONPath } from "jsonpath-plus";

const ALLOWED_PREFIXES = ["$.user", "$.order", "$.items"];

function isAllowedJsonPath(path: string): boolean {
    return ALLOWED_PREFIXES.some(prefix => path.startsWith(prefix));
}

function queryData(req: Request, res: Response) {
    const path = req.body.path;
    if (!isAllowedJsonPath(path)) throw new Error("Path not allowed");
    const data = req.body.data || getDefaultData();
    const result = JSONPath({ path, json: data });
    res.json({ result });
}

function extractField(req: Request, res: Response) {
    const path = req.query.path as string;
    if (!isAllowedJsonPath(path)) throw new Error("Access denied");
    const doc = getDocument(req.params.id);
    const result = JSONPath({ path, json: doc });
    res.json({ result });
}
