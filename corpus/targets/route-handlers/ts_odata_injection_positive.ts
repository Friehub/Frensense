// [frensense]
// observation: User-controlled input is passed as an OData filter or orderby parameter, allowing injection of arbitrary filter expressions that bypass restrictions.
// impact: An attacker can inject OData operators (eq, ne, gt, lt, etc.) and logical operators (and, or, not) to access unauthorized data or perform enumeration attacks.
// improvement: Validate the filter expression against an allowlist of permitted fields and operators.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High
// runtime_probe: sqli

import { createClient } from "odata-client";

const client = createClient("https://api.example.com/odata");

async function getProducts(req: Request, res: Response) {
    const filter = req.query.$filter as string;
    const result = await client.get("Products", { filter });
    res.json(result);
}

async function searchOrders(req: Request, res: Response) {
    const orderby = req.body.$orderby;
    const result = await client.get("Orders", {
        filter: req.body.$filter,
        orderby: orderby,
    });
    res.json(result);
}
