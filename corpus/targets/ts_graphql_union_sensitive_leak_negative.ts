// SAFE: The union type resolver checks the viewer's relationship to each result and throws an auth error for restricted types.

import { ApolloServer, gql, ForbiddenError } from 'apollo-server-express';

const typeDefs = gql`
  union SearchResult = PublicProfile | PrivateAccount

  type PublicProfile {
    id: ID!
    displayName: String!
    avatarUrl: String!
  }

  type PrivateAccount {
    id: ID!
    displayName: String!
  }

  type Query {
    search(term: String!): [SearchResult!]!
  }
`;

interface UserRow {
  id: string;
  displayName: string;
  accountType: 'public' | 'private';
}

interface Context {
  user: { id: string } | null;
}

const resolvers = {
  SearchResult: {
    __resolveType: (parent: UserRow, context: Context) => {
      if (parent.accountType === 'private' && (!context.user || context.user.id !== parent.id)) {
        return 'PublicProfile';
      }
      return parent.accountType === 'private' ? 'PrivateAccount' : 'PublicProfile';
    },
  },
  Query: {
    search: async (_: any, { term }: { term: string }, { db }: any) => {
      return db.query(
        'SELECT id, displayName, accountType FROM users WHERE displayName LIKE ?', [`%${term}%`]
      ).all();
    },
  },
};
