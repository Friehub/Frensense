// SAFE: The parent resolver never returns sensitive types for unauthenticated users, and field resolvers add a second layer of auth.

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
    email: String!
  }

  type Query {
    search(term: String!): [SearchResult!]!
  }
`;

interface UserRow {
  id: string;
  displayName: string;
  avatarUrl: string;
  email: string;
  accountType: 'public' | 'private';
}

interface Context {
  user: { id: string } | null;
}

const resolvers = {
  SearchResult: {
    __resolveType: (parent: UserRow) => {
      return parent.accountType === 'private' ? 'PrivateAccount' : 'PublicProfile';
    },
  },
  Query: {
    search: async (_: any, { term }: { term: string }, { user, db }: Context & { db: any }) => {
      const rows: UserRow[] = db.query(
        'SELECT id, displayName, avatarUrl, email, accountType FROM users WHERE displayName LIKE ?',
        [`%${term}%`]
      ).all();
      return rows.filter((r) => r.accountType === 'public' || (user && user.id === r.id));
    },
  },
  PrivateAccount: {
    email: (parent: UserRow, _: any, context: Context) => {
      if (!context.user || context.user.id !== parent.id) {
        throw new ForbiddenError('Can only view your own private account email');
      }
      return parent.email;
    },
  },
};
