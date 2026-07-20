// SAFE: Whitelisted allowed query fields and stripped MongoDB operators from user input before constructing the query.

import { MongoClient } from "mongodb";

const ALLOWED_FIELDS = new Set(["username", "email", "status", "role"]);

function sanitizeQuery(input: Record<string, unknown>): Record<string, unknown> {
    const clean: Record<string, unknown> = {};
    for (const key of Object.keys(input)) {
        if (ALLOWED_FIELDS.has(key) && !key.startsWith("$")) {
            clean[key] = input[key];
        }
    }
    return clean;
}

async function findUser(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const query = sanitizeQuery(req.body);
    const user = await db.collection("users").findOne(query);
    res.json(user);
}

async function updateItems(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const filter = sanitizeQuery(req.body.filter || {});
    const data = sanitizeQuery(req.body.data || {});
    await db.collection("items").updateMany(filter, { $set: data });
    res.json({ success: true });
}
