// SAFE: Connection-level authentication is verified during the WebSocket handshake, and subscription resolvers delegate to a channel membership service.

import { ApolloServer, gql, PubSub, ForbiddenError } from 'apollo-server-express';

const pubsub = new PubSub();

const typeDefs = gql`
  type Message {
    id: ID!
    text: String!
    channelId: String!
  }

  type Subscription {
    messageAdded(channelId: String!): Message
  }
`;

async function verifyChannelAccess(userId: string, channelId: string): Promise<boolean> {
  const membership = await db.query(
    'SELECT 1 FROM channel_members WHERE userId = ? AND channelId = ?',
    [userId, channelId]
  ).get();
  return !!membership;
}

const resolvers = {
  Subscription: {
    messageAdded: {
      subscribe: async (_: any, { channelId }: { channelId: string }, context: any) => {
        if (!context.user) throw new ForbiddenError('Not authenticated');
        const hasAccess = await verifyChannelAccess(context.user.id, channelId);
        if (!hasAccess) throw new ForbiddenError('Not a member of this channel');
        return pubsub.asyncIterator(`MESSAGE_ADDED_${channelId}`);
      },
    },
  },
};
