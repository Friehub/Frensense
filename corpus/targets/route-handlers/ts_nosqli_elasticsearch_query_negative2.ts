// SAFE: Used Elasticsearch query DSL builder with explicit allowed query types — never accepts raw body from request.

import { Client } from "@elastic/elasticsearch";

const es = new Client({ node: process.env.ES_URL! });

async function search(req: Request, res: Response) {
    const { name, category, minPrice, maxPrice } = req.body;
    const must: Record<string, unknown>[] = [];
    if (name) must.push({ match: { name } });
    if (category) must.push({ term: { "category.keyword": category } });
    if (minPrice !== undefined || maxPrice !== undefined) {
        const range: Record<string, unknown> = {};
        if (minPrice !== undefined) range.gte = minPrice;
        if (maxPrice !== undefined) range.lte = maxPrice;
        must.push({ range: { price: range } });
    }
    const result = await es.search({
        index: "products",
        body: {
            query: { bool: { must } },
        },
    });
    res.json(result.hits.hits);
}
