// SAFE: Field-level cost mapping with per-field complexity weights
import { ApolloServer } from 'apollo-server-express';
import { costAnalysis } from 'graphql-cost-analysis';

const costMap = new Map([
  ['Query.users', { cost: 5 }],
  ['Query.orders', { cost: 5 }],
  ['User.friends', { cost: 3 }],
  ['Order.lineItems', { cost: 2 }],
  ['Mutation.createOrder', { cost: 10 }],
]);

const server = new ApolloServer({
  typeDefs,
  resolvers,
  validationRules: [
    costAnalysis({ maximumCost: 100, defaultCost: 1, costMap })
  ],
});

app.use('/graphql', server.getMiddleware());
