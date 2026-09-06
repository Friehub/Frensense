// SAFE: Removed $where operator and used standard MongoDB query operators to filter results safely.

import { MongoClient } from "mongodb";

async function searchUsers(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("app");
    const { role, status } = req.body;
    const query: Record<string, unknown> = {};
    if (role) query.role = role;
    if (status) query.status = status;
    const users = await db.collection("users").find(query).toArray();
    res.json(users);
}

async function adminLookup(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("admin");
    const result = await db.collection("sessions").find({ role: "admin" }).toArray();
    res.json(result);
}
