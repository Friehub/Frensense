// SAFE: Used a static GraphQL query with variables instead of building query strings from user input.

import { graphql, buildSchema } from "graphql";

const schema = buildSchema(`
    type Query { user(id: ID!): User }
    type User { id: ID, name: String, email: String, role: String }
`);

const STATIC_QUERY = `query GetUser($id: ID!) { user(id: $id) { id name email role } }`;

async function handleGraphQL(req: Request, res: Response) {
    const result = await graphql({
        schema,
        source: STATIC_QUERY,
        variableValues: { id: req.body.id },
        rootValue: root,
    });
    res.json(result);
}

async function customQuery(req: Request, res: Response) {
    const result = await graphql({
        schema,
        source: `query GetUser($id: ID!) { user(id: $id) { id name email } }`,
        variableValues: { id: req.body.id },
        rootValue: root,
    });
    res.json(result);
}
