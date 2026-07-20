// SAFE: Uses SQL JOINs to fetch related data in the parent query, avoiding per-row field resolver round-trips.

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
    authorName: String!
  }
  type Query {
    posts: [Post!]!
  }
`;

interface PostRow {
  id: string;
  title: string;
  authorName: string;
}

interface AuthorRow {
  id: string;
  name: string;
}

const resolvers = {
  Query: {
    posts: async (_: any, __: any, { db }: any) => {
      const rows: PostRow[] = db.query(
        'SELECT p.id, p.title, a.name AS authorName FROM posts p JOIN authors a ON p.authorId = a.id'
      ).all();
      return rows;
    },
  },
  Author: {
    posts: async (parent: AuthorRow, _: any, { db }: any) => {
      const rows: PostRow[] = db.query(
        'SELECT p.id, p.title, a.name AS authorName FROM posts p JOIN authors a ON p.authorId = a.id WHERE a.id = ?',
        [parent.id]
      ).all();
      return rows;
    },
  },
};
