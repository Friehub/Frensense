// SAFE: Query depth is limited to prevent nested query attacks
import { ApolloServer } from 'apollo-server-express';
import depthLimit from 'graphql-depth-limit';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  validationRules: [depthLimit(7)],
});

app.use('/graphql', server.getMiddleware());
