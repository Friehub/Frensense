// [frensense]
// observation: Some GraphQL field resolvers lack authorization checks, exposing sensitive fields that should be restricted based on user role or ownership.
// impact: An attacker can query sensitive fields (e.g., email, ssn, passwordHash, internalNotes) on resources they have access to, bypassing field-level access controls.
// improvement: Apply authorization checks in every field resolver that returns sensitive data, or use a schema directive-based approach for field-level access control.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { ApolloServer, gql } from 'apollo-server-express';

const typeDefs = gql`
  type User {
    id: ID!
    name: String!
    email: String!
    ssn: String!
    passwordHash: String!
    internalNotes: String!
  }
  type Query {
    user(id: ID!): User
  }
`;

const resolvers = {
  Query: {
    user: async (_, { id }, { db }) => {
      return db.prepare('SELECT * FROM users WHERE id = ?').bind(id).first();
    }
  },
  User: {
    email: (parent) => parent.email,
    ssn: (parent) => parent.ssn,
    passwordHash: (parent) => parent.passwordHash,
    internalNotes: (parent) => parent.internalNotes,
  }
};
