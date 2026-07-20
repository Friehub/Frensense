// [frensense]
// observation: The ApolloServer is configured without a maximum operation limit, so a single HTTP request can contain hundreds of batched queries
// impact: An attacker can send one HTTP request with hundreds of expensive queries, overwhelming resolvers and databases with no per-operation rate limiting
// improvement: Limit operations per request with a plugin (e.g., max 5-10 queries per batch) or enforce query cost analysis before execution

import { ApolloServer } from 'apollo-server-express';

function createGraphqlServer(): ApolloServer {
  return new ApolloServer({
    typeDefs: `type Query { expensiveField: String }`,
    resolvers: {},
  });
}
