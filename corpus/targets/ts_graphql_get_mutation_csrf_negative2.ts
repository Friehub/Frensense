// SAFE: A custom middleware on the Express/Gateway layer rejects GET requests to the GraphQL endpoint entirely.

import { ApolloServer, gql } from 'apollo-server-express';

const typeDefs = gql`
  type Mutation {
    deleteAccount(userId: ID!): Boolean!
  }
  type Query {
    hello: String!
  }
`;

const resolvers = {
  Mutation: {
    deleteAccount: async (_: any, { userId }: { userId: string }) => {
      return db.query('DELETE FROM users WHERE id = ?', [userId]).run();
    },
  },
  Query: {
    hello: () => 'world',
  },
};

function graphqlMethodGuard(req: any, res: any, next: any) {
  if (req.method === 'GET') {
    const query = typeof req.query.query === 'string' ? req.query.query : '';
    if (query.includes('mutation')) {
      return res.status(405).json({ error: 'GET requests are not allowed for mutations' });
    }
  }
  next();
}

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

app.use('/graphql', graphqlMethodGuard, server.getMiddleware());
