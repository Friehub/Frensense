// SAFE alternative: use a persisted operations whitelist
import { ApolloServer } from '@apollo/server';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  introspection: false,
  persistedQueries: { ttl: 900 },
});
