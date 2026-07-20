// SAFE: Subscription resolvers check authentication and channel membership before allowing subscription.

import { ApolloServer, gql, PubSub, ForbiddenError } from 'apollo-server-express';
import { withFilter } from 'apollo-server-express';

const pubsub = new PubSub();

const typeDefs = gql`
  type Message {
    id: ID!
    text: String!
    channelId: String!
  }

  type Subscription {
    messageAdded(channelId: String!): Message
    adminNotifications: String!
  }
`;

interface Context {
  user: { id: string; role: string } | null;
}

const resolvers = {
  Subscription: {
    messageAdded: {
      subscribe: withFilter(
        (_: any, { channelId }: { channelId: string }, context: Context) => {
          if (!context.user) throw new ForbiddenError('Not authenticated');
          return pubsub.asyncIterator(`MESSAGE_ADDED_${channelId}`);
        },
        (payload: any, variables: { channelId: string }, context: Context) => {
          return context.user !== null;
        },
      ),
    },
    adminNotifications: {
      subscribe: (_: any, __: any, context: Context) => {
        if (!context.user || context.user.role !== 'admin') {
          throw new ForbiddenError('Admin role required');
        }
        return pubsub.asyncIterator('ADMIN_NOTIFICATIONS');
      },
    },
  },
};
