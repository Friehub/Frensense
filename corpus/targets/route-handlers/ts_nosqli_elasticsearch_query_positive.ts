// [frensense]
// observation: The entire request body is passed directly as an Elasticsearch query body without any validation, allowing arbitrary query injection.
// impact: An attacker can craft queries that bypass security filters, access restricted indices, trigger expensive aggregations (DoS), or extract data via search templates.
// improvement: Validate and restrict the query structure against an allowlist of permitted fields and query types before sending to Elasticsearch.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { Client } from "@elastic/elasticsearch";

const es = new Client({ node: process.env.ES_URL! });

async function search(req: Request, res: Response) {
    const queryBody = req.body;
    const result = await es.search({
        index: "products",
        body: queryBody,
    });
    res.json(result.hits.hits);
}

async function customAggregation(req: Request, res: Response) {
    const aggBody = req.body.aggregation;
    const result = await es.search({
        index: "logs",
        body: {
            query: { match_all: {} },
            aggs: aggBody,
        },
    });
    res.json(result.aggregations);
}
