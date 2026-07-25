// [frensense]
// observation: GraphQL query complexity is not analyzed, allowing expensive queries that join multiple large datasets to execute unchecked.
// impact: An attacker can craft queries that join multiple resource types, requesting large lists with all fields, causing high CPU/database load and denial of service.
// improvement: Implement query cost analysis to reject queries above a complexity threshold.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { ApolloServer } from 'apollo-server-express';

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

app.use('/graphql', server.getMiddleware());
