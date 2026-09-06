// SAFE: Used text index with $text search instead of $regex, preventing regex injection entirely.

import { MongoClient } from "mongodb";

async function ensureTextIndex() {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("shop");
    await db.collection("products").createIndex({ name: "text" });
    await client.close();
}

async function searchProducts(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("shop");
    const searchTerm = req.query.q as string;
    const products = await db.collection("products").find({
        $text: { $search: searchTerm }
    }).toArray();
    res.json(products);
}
