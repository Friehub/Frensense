// SAFE: Introspection only allowed for authenticated admin users via plugin
import { ApolloServer } from 'apollo-server-express';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  introspection: false,
  plugins: [{
    async requestDidStart({ request, context }) {
      if (request.operationName === 'IntrospectionQuery' || request.query?.includes('__schema')) {
        if (context?.user?.role !== 'admin') {
          throw new Error('Introspection not allowed');
        }
      }
    }
  }],
});
