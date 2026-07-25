// [frensense]
// observation: User-controlled input is passed directly to the $regex operator in a MongoDB query, allowing ReDoS attacks and query manipulation.
// impact: An attacker can inject a malicious regex pattern causing excessive CPU consumption (ReDoS) or craft patterns that match unintended documents.
// improvement: Escape special regex characters in user input and limit regex complexity by rejecting patterns with nested quantifiers.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { MongoClient } from "mongodb";

async function searchProducts(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("shop");
    const searchTerm = req.query.q as string;
    const products = await db.collection("products").find({
        name: { $regex: searchTerm, $options: "i" }
    }).toArray();
    res.json(products);
}

async function filterByPattern(req: Request, res: Response) {
    const client = new MongoClient(process.env.MONGO_URI!);
    await client.connect();
    const db = client.db("shop");
    const { field, pattern, flags } = req.body;
    const docs = await db.collection("logs").find({
        [field]: { $regex: pattern, $options: flags || "" }
    }).toArray();
    res.json(docs);
}
