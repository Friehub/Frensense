// SAFE: GET requests that contain mutation operations are rejected before reaching the resolver.

import { ApolloServer, gql } from 'apollo-server-express';

const typeDefs = gql`
  type Mutation {
    deleteAccount(userId: ID!): Boolean!
    transferFunds(amount: Float!, toAccount: String!): Boolean!
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
    transferFunds: async (_: any, { amount, toAccount }: { amount: number; toAccount: string }) => {
      return db.query('UPDATE accounts SET balance = balance - ? WHERE id = ?', [amount, toAccount]).run();
    },
  },
  Query: {
    hello: () => 'world',
  },
};

function rejectMutationGetRequests(req: any, res: any, next: any) {
  if (req.method === 'GET' && req.body?.query?.trim().startsWith('mutation')) {
    return res.status(405).json({ error: 'Mutations must use POST' });
  }
  next();
}

app.use('/graphql', rejectMutationGetRequests);

const server = new ApolloServer({
  typeDefs,
  resolvers,
  csrfPrevention: true,
});
