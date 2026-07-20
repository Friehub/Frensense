// [frensense]
// observation: User input is used as a computed property key or operator in a MongoDB query, enabling NoSQL injection through operator manipulation.
// impact: An attacker can inject MongoDB operators like $ne, $gt, $regex, or $where to bypass authentication, extract data, or perform unauthorized operations.
// improvement: Validate that the operator or field name matches an allowlist before using it as a computed key.

import { MongoClient } from "mongodb";

async function login(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const { username, password } = req.body;
    const user = await db.collection("users").findOne({ [req.body.operator]: { username, password } });
    if (user) {
        res.json({ token: generateToken(user) });
    } else {
        res.status(401).json({ error: "Invalid credentials" });
    }
}

async function getItems(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const query = { ...req.body.filter };
    const items = await db.collection("items").find(query).toArray();
    res.json(items);
}
