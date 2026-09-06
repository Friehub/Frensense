// SAFE: User input is validated against an allowlist before being used. Only safe properties are merged.

import express from "express";

const app = express();
const ALLOWED_KEYS = new Set(["theme", "locale", "timezone"]);

function sanitize(input: Record<string, unknown>): Record<string, unknown> {
    const safe: Record<string, unknown> = {};
    for (const key of Object.keys(input)) {
        if (ALLOWED_KEYS.has(key) && !key.startsWith("__")) {
            safe[key] = input[key];
        }
    }
    return safe;
}

app.post("/update", (req: express.Request, res: express.Response) => {
    const config = { name: "default", role: "user" };
    const safe = sanitize(req.body);
    Object.assign(config, safe);
    res.json({ status: "updated" });
});