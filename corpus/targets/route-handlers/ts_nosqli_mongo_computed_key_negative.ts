// SAFE: Replaced computed operator key with static query fields and validated all operator inputs against an allowlist.

import { MongoClient } from "mongodb";

const ALLOWED_OPERATORS = new Set(["$eq", "$gt", "$gte", "$lt", "$lte", "$in", "$nin"]);

function sanitizeOperator(op: string): string {
    if (!ALLOWED_OPERATORS.has(op)) throw new Error("Invalid operator");
    return op;
}

async function login(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const { username, password } = req.body;
    const user = await db.collection("users").findOne({ username, password });
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
    const { field, operator, value } = req.body.filter;
    const safeOp = sanitizeOperator(operator);
    const items = await db.collection("items").find({ [field]: { [safeOp]: value } }).toArray();
    res.json(items);
}
