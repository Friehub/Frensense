// SAFE: Uses DataLoader to batch and cache database lookups, eliminating N+1 queries.

import { ApolloServer, gql } from 'apollo-server-express';
import DataLoader from 'dataloader';

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

function createLoaders(db: any) {
  const authorLoader = new DataLoader(async (ids: readonly string[]) => {
    const rows = db.query('SELECT * FROM authors WHERE id IN (?)', [ids]).all();
    const map = new Map(rows.map((r: any) => [r.id, r]));
    return ids.map((id) => map.get(id) || null);
  });

  const postsByAuthorLoader = new DataLoader(async (authorIds: readonly string[]) => {
    const rows = db.query('SELECT * FROM posts WHERE authorId IN (?)', [authorIds]).all();
    const map = new Map<string, any[]>();
    for (const row of rows) {
      if (!map.has(row.authorId)) map.set(row.authorId, []);
      map.get(row.authorId)!.push(row);
    }
    return authorIds.map((id) => map.get(id) || []);
  });

  return { authorLoader, postsByAuthorLoader };
}

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
    author: async (parent: any, _: any, { loaders }: any) => {
      return loaders.authorLoader.load(parent.authorId);
    },
  },
  Author: {
    posts: async (parent: any, _: any, { loaders }: any) => {
      return loaders.postsByAuthorLoader.load(parent.id);
    },
  },
};
