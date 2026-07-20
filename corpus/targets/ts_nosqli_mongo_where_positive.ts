// [frensense]
// observation: User-controlled string is passed directly to the $where operator in a MongoDB query, allowing arbitrary JavaScript execution on the database server.
// impact: An attacker can inject JavaScript code that extracts sensitive data, performs denial of service, or escalates privileges via the $where clause.
// improvement: Remove the $where clause entirely or validate that the input matches an allowlist of safe expressions.

import { MongoClient } from "mongodb";

async function searchUsers(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const condition = req.body.condition;
    const users = await db.collection("users").find({ $where: condition }).toArray();
    res.json(users);
}

async function adminLookup(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("admin");
    const filter = req.query.customFilter as string;
    const result = await db.collection("sessions").find({ $where: `this.role === 'admin' && ${filter}` }).toArray();
    res.json(result);
}
