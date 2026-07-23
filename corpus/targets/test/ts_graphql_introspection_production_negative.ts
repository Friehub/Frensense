// SAFE: Introspection disabled in production
import { ApolloServer } from 'apollo-server-express';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  introspection: process.env.NODE_ENV !== 'production',
});

app.use('/graphql', server.getMiddleware());
