// [frensense]
// observation: A GraphQL field resolver fetches related data one-at-a-time from the database, causing N+1 query amplification where each parent result triggers a separate database query.
// impact: Querying a list of N items generates N+1 database round-trips, causing severe performance degradation and potential denial of service under moderate load.
// improvement: Use DataLoader to batch and cache per-request database lookups, or include the related data in the parent query via JOINs.

import { ApolloServer, gql } from 'apollo-server-express';

const typeDefs = gql`
  type Author {
    id: ID!
    name: String!
    posts: [Post!]!
  }
  type Post {
    id: ID!
    title: String!
    author: Author!
  }
  type Query {
    posts: [Post!]!
    authors: [Author!]!
  }
`;

const resolvers = {
  Query: {
    posts: async (_: any, __: any, { db }: any) => {
      return db.query('SELECT * FROM posts').all();
    },
    authors: async (_: any, __: any, { db }: any) => {
      return db.query('SELECT * FROM authors').all();
    },
  },
  Post: {
    author: async (parent: any, _: any, { db }: any) => {
      return db.query('SELECT * FROM authors WHERE id = ?', [parent.authorId]).get();
    },
  },
  Author: {
    posts: async (parent: any, _: any, { db }: any) => {
      return db.query('SELECT * FROM posts WHERE authorId = ?', [parent.id]).all();
    },
  },
};
