// [frensense]
// observation: The ApolloServer is configured without a depth limit validator, so deeply nested queries can reach resolvers without restriction
// impact: An attacker can craft a deeply nested query (e.g., user → friends → user → friends → ...) that causes exponential resolver calls and database lookups, leading to denial of service
// improvement: Apply graphql-depth-limit or a custom depth validator in the ApolloServer validationRules
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { ApolloServer } from 'apollo-server-express';

function createGraphqlServer(): ApolloServer {
  return new ApolloServer({
    typeDefs: `type Query { user(id: ID!): User } type User { friends: [User] }`,
    resolvers: {},
  });
}
