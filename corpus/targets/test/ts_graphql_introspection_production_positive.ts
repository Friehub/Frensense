// [frensense]
// observation: The ApolloServer enables introspection in production, exposing __schema and __type queries to unauthenticated clients
// impact: An attacker can enumerate the entire GraphQL schema including undocumented mutations, internal types, and field arguments to craft precise attacks
// improvement: Set introspection to false in production, or conditionally enable it only for authenticated admin users via a plugin
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { ApolloServer } from 'apollo-server-express';

function createGraphqlServer(): ApolloServer {
  return new ApolloServer({
    typeDefs: `type Query { secretData: String }`,
    resolvers: {},
    introspection: true,
  });
}
