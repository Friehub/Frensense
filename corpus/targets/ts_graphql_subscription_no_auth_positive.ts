// [frensense]
// observation: A GraphQL subscription resolver lacks authentication checks, allowing any connected WebSocket client to receive real-time updates for sensitive events.
// impact: Unauthenticated clients can subscribe to private event streams (e.g., admin notifications, user-specific updates), leaking sensitive data in real time.
// improvement: Authenticate the WebSocket connection during the subscription handshake and verify authorization in each subscription resolver.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { ApolloServer, gql, PubSub } from 'apollo-server-express';
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

const resolvers = {
  Subscription: {
    messageAdded: {
      subscribe: (_: any, { channelId }: { channelId: string }) => {
        return pubsub.asyncIterator(`MESSAGE_ADDED_${channelId}`);
      },
    },
    adminNotifications: {
      subscribe: () => {
        return pubsub.asyncIterator('ADMIN_NOTIFICATIONS');
      },
    },
  },
};
