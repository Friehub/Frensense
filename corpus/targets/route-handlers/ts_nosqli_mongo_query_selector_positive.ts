// [frensense]
// observation: The entire request body is passed directly as a MongoDB query selector without any sanitization or schema validation.
// impact: An attacker can inject operators like $gt, $ne, $regex, or $where to escalate privileges, bypass authentication, or extract all documents from a collection.
// improvement: Strip query operators from user input or whitelist allowed fields before constructing the query.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { MongoClient } from "mongodb";

async function findUser(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const query = req.body;
    const user = await db.collection("users").findOne(query);
    res.json(user);
}

async function updateItems(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const filter = Object.assign({}, req.body.filter);
    const update = { $set: req.body.data };
    await db.collection("items").updateMany(filter, update);
    res.json({ success: true });
}
