// SAFE: Query complexity analysis limits expensive queries
import { ApolloServer } from 'apollo-server-express';
import { createComplexityLimitRule } from 'graphql-validation-complexity';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  validationRules: [createComplexityLimitRule(1000)],
});

app.use('/graphql', server.getMiddleware());
