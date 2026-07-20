// SAFE: Escape special regex characters in user input using regex escaping, preventing injection of malicious patterns.

import { MongoClient } from "mongodb";

function escapeRegex(str: string): string {
    return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

async function searchProducts(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("shop");
    const searchTerm = escapeRegex(req.query.q as string);
    const products = await db.collection("products").find({
        name: { $regex: searchTerm, $options: "i" }
    }).toArray();
    res.json(products);
}

async function filterByPattern(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("shop");
    const { field, pattern } = req.body;
    const escapedPattern = escapeRegex(pattern);
    const docs = await db.collection("logs").find({
        [field]: { $regex: escapedPattern }
    }).toArray();
    res.json(docs);
}
