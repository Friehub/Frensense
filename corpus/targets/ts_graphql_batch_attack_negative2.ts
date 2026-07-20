// SAFE: Disables batching entirely and uses query cost analysis
import { ApolloServer } from 'apollo-server-express';
import { costAnalysis } from 'graphql-cost-analysis';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  validationRules: [
    costAnalysis({ maximumCost: 1000, defaultCost: 1, costMap: new Map() })
  ],
});

app.post('/graphql', (req, res, next) => {
  if (Array.isArray(req.body)) return res.status(400).json({ error: 'Batching not allowed' });
  next();
}, server.getMiddleware());
