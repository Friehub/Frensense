// SAFE: Replaced user-supplied JMESPath expressions with named queries from a predefined registry.

import { search } from "jmespath";

const QUERY_REGISTRY: Record<string, string> = {
    "userNames": "users[*].name",
    "activeUsers": "users[?status=='active']",
    "totalRevenue": "sum(orders[].total)",
    "productCatalog": "products[].{name: name, price: price}",
};

function queryData(req: Request, res: Response) {
    const queryName = req.body.query;
    const expression = QUERY_REGISTRY[queryName];
    if (!expression) throw new Error("Unknown query");
    const data = req.body.data || getDefaultData();
    const result = search(data, expression);
    res.json({ result });
}

function advancedSearch(req: Request, res: Response) {
    const queryName = req.query.q as string;
    const expression = QUERY_REGISTRY[queryName];
    if (!expression) throw new Error("Unknown query");
    const docs = getDocuments();
    const results = docs.map(d => search(d, expression));
    res.json(results);
}
