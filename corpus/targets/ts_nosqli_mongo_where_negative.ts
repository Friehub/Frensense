// SAFE: User input is validated and checked for MongoDB operators before being used as a query filter.

import express from "express";
import { Router } from "express";

const router = Router();
const FORBIDDEN_OPERATORS = ["$where", "$regex", "$gt", "$ne", "$nin", "$or"];

function sanitizeFilter(input: Record<string, unknown>): Record<string, unknown> {
    const safe: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(input)) {
        if (key.startsWith("$")) continue;
        if (typeof value === "object" && value !== null && !Array.isArray(value)) {
            safe[key] = sanitizeFilter(value as Record<string, unknown>);
        } else {
            safe[key] = value;
        }
    }
    return safe;
}

router.post("/find", async (req: express.Request, res: express.Response) => {
    const filter = sanitizeFilter(req.body.filter);
    const results = await db.collection("users").find(filter).toArray();
    res.json(results);
});