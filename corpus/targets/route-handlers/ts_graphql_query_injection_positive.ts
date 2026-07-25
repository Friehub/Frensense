// [frensense]
// observation: User-controlled input is used to construct a GraphQL query or mutation string, allowing arbitrary field access and query injection.
// impact: An attacker can inject fields that access unauthorized data, perform deeply nested queries for DoS, or bypass field-level authorization in GraphQL resolvers.
// improvement: Use a static GraphQL query with arguments instead of building query strings from user input.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High
// runtime_probe: sqli

import { graphql, buildSchema } from "graphql";

const schema = buildSchema(`
    type Query { user(id: ID!): User }
    type User { id: ID, name: String, email: String, role: String }
`);

async function handleGraphQL(req: Request, res: Response) {
    const query = req.body.query;
    const result = await graphql({ schema, source: query, rootValue: root });
    res.json(result);
}

async function customQuery(req: Request, res: Response) {
    const fields = req.body.fields.join(" ");
    const query = `{ user(id: "${req.body.id}") { ${fields} } }`;
    const result = await graphql({ schema, source: query, rootValue: root });
    res.json(result);
}
