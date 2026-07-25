// [frensense]
// observation: The GraphQL endpoint accepts GET requests for mutation operations. An attacker can craft a <img> or <script> tag that triggers a GET request to perform a state-changing mutation without the victim's consent.
// impact: Cross-Site Request Forgery (CSRF) — an external site can execute mutations on behalf of an authenticated user by embedding a simple image or link that triggers a GET request to the GraphQL endpoint.
// improvement: Reject GET requests for mutation operations, or require CSRF tokens and custom headers for all mutations.
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

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

const server = new ApolloServer({
  typeDefs,
  resolvers,
  csrfPrevention: false,
});
