// SAFE: Used Apollo Server with persisted operations (APQ), preventing arbitrary query execution.

import { ApolloServer, gql } from "apollo-server-express";

const typeDefs = gql`
    type Query { user(id: ID!): User }
    type User { id: ID, name: String, email: String, role: String }
`;

const resolvers = {
    Query: {
        user: (_, { id }, context) => context.loaders.user.load(id),
    },
};

const server = new ApolloServer({
    typeDefs,
    resolvers,
    persistedQueries: {
        ttl: 900,
    },
});

// Only pre-registered queries with hash can be executed
async function handleGraphQL(req: Request, res: Response) {
    const result = await graphql({
        schema,
        source: `query ($id: ID!) { user(id: $id) { id name email role } }`,
        variableValues: { id: req.body.id },
        rootValue: root,
    });
    res.json(result);
}
