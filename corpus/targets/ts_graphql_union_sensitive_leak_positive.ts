// [frensense]
// observation: A GraphQL union or interface type has a resolver that returns sensitive data for specific member types without checking whether the requesting user is authorized to see that data.
// impact: An attacker can query a union field and, by using inline fragments, extract sensitive fields from restricted types that should not be visible to their role.
// improvement: Add authorization checks in the type resolver or in each member type's field resolvers before returning sensitive data.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { ApolloServer, gql } from 'apollo-server-express';

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
    ssn: String!
    internalNotes: String!
    isAdmin: Boolean!
  }

  type Query {
    search(term: String!): [SearchResult!]!
  }
`;

interface UserRow {
  id: string;
  displayName: string;
  email: string;
  ssn: string;
  internalNotes: string;
  isAdmin: boolean;
  accountType: 'public' | 'private';
}

const resolvers = {
  SearchResult: {
    __resolveType: (parent: UserRow) => {
      return parent.accountType === 'private' ? 'PrivateAccount' : 'PublicProfile';
    },
  },
  Query: {
    search: async (_: any, { term }: { term: string }, { db }: any) => {
      return db.query(
        'SELECT * FROM users WHERE displayName LIKE ?', [`%${term}%`]
      ).all();
    },
  },
  PrivateAccount: {
    email: (parent: UserRow) => parent.email,
    ssn: (parent: UserRow) => parent.ssn,
    internalNotes: (parent: UserRow) => parent.internalNotes,
    isAdmin: (parent: UserRow) => parent.isAdmin,
  },
};
