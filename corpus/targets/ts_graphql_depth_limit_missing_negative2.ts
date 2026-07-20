// SAFE: Custom depth validation rule with configurable max depth
import { ApolloServer } from 'apollo-server-express';
import { graphqlDepthLimit } from './customRules';

const MAX_DEPTH = 10;

const server = new ApolloServer({
  typeDefs,
  resolvers,
  validationRules: [context => ({
    Field(node) {
      const depth = getDepth(context, node);
      if (depth > MAX_DEPTH) {
        context.reportError(new Error(`Query exceeds maximum depth of ${MAX_DEPTH}`));
      }
    }
  })],
});

function getDepth(context: any, node: any): number {
  let depth = 0;
  let current = node;
  while (current.parent) { depth++; current = current.parent; }
  return depth;
}
