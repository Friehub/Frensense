// [frensense]
// observation: GraphQL schema introspection (__schema, __type) is enabled in production, exposing the entire API schema including all types, fields, arguments, and resolvers.
// impact: Attackers can discover hidden fields (e.g., isAdmin, passwordHash, internalNote), undocumented mutations, and the full data model. This is the GraphQL equivalent of dumping the entire API documentation with internal-only endpoints.
// improvement: Disable introspection in production. Use a whitelist of allowed operations or separate schemas for internal vs external use.

import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  // VULNERABLE: introspection enabled in production
  introspection: true,
});

await startStandaloneServer(server, { listen: { port: 4000 } });
