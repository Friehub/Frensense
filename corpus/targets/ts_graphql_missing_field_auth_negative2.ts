// SAFE: Uses GraphQL directives for declarative field-level authorization
import { ApolloServer, gql } from 'apollo-server-express';

const typeDefs = gql`
  directive @auth(requires: Role = ADMIN) on OBJECT | FIELD_DEFINITION
  enum Role { ADMIN USER }

  type User {
    id: ID!
    name: String!
    email: String @auth(requires: ADMIN)
    ssn: String @auth(requires: ADMIN)
    passwordHash: String @auth(requires: ADMIN)
  }

  type Query {
    user(id: ID!): User @auth(requires: USER)
    me: User @auth(requires: USER)
  }
`;

const resolvers = {
  Query: {
    user: async (_, { id }, { db }) => db.prepare('SELECT * FROM users WHERE id = ?').bind(id).first(),
    me: async (_, __, { db, user }) => db.prepare('SELECT * FROM users WHERE id = ?').bind(user?.sub).first(),
  }
};
