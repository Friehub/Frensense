// SAFE: Constructed a safe Elasticsearch query that only allows specific fields and match values from user input.

import { Client } from "@elastic/elasticsearch";

const es = new Client({ node: process.env.ES_URL! });

async function search(req: Request, res: Response) {
    const { field, value } = req.body;
    const ALLOWED_FIELDS = ["name", "description", "category", "price"];
    if (!ALLOWED_FIELDS.includes(field)) {
        res.status(400).json({ error: "Invalid search field" });
        return;
    }
    const result = await es.search({
        index: "products",
        body: {
            query: {
                match: {
                    [field]: value,
                },
            },
        },
    });
    res.json(result.hits.hits);
}

async function customAggregation(req: Request, res: Response) {
    const result = await es.search({
        index: "logs",
        body: {
            query: { match_all: {} },
            aggs: {
                by_status: {
                    terms: { field: "status.keyword" },
                },
            },
        },
    });
    res.json(result.aggregations);
}
