// SAFE: Sensitive fields have authorization checks in their resolvers
import { ApolloServer, gql } from 'apollo-server-express';

const typeDefs = gql`
  type User {
    id: ID!
    name: String!
    email: String
    ssn: String
  }
  type Query {
    user(id: ID!): User
    me: User
  }
`;

const resolvers = {
  Query: {
    user: async (_, { id }, { db, user }) => {
      const record = await db.prepare('SELECT * FROM users WHERE id = ?').bind(id).first();
      if (!record) return null;
      if (record.id !== user?.id && user?.role !== 'admin') return { id: record.id, name: record.name };
      return record;
    },
    me: async (_, __, { db, user }) => {
      if (!user) return null;
      return db.prepare('SELECT * FROM users WHERE id = ?').bind(user.sub).first();
    }
  },
  User: {
    email: (parent, _, { user }) => parent.id === user?.sub || user?.role === 'admin' ? parent.email : null,
    ssn: (parent, _, { user }) => user?.role === 'admin' ? parent.ssn : null,
  }
};
