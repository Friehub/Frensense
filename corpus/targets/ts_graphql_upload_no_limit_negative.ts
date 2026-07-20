// SAFE: Each upload is checked against a maximum byte size before the stream is fully consumed.

import { ApolloServer, gql, ApolloError } from 'apollo-server-express';
import { GraphQLUpload } from 'graphql-upload-minimal';
import { createWriteStream } from 'fs';
import { pipeline } from 'stream/promises';

const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10 MB

const typeDefs = gql`
  scalar Upload

  type Mutation {
    uploadProfilePhoto(file: Upload!): Boolean!
  }
`;

interface UploadArgs {
  file: { createReadStream: () => NodeJS.ReadableStream; filename: string; mimetype: string };
}

const resolvers = {
  Upload: GraphQLUpload,
  Mutation: {
    uploadProfilePhoto: async (_: any, { file }: UploadArgs) => {
      const { createReadStream, filename } = await file;
      const stream = createReadStream();
      let totalBytes = 0;

      for await (const chunk of stream) {
        totalBytes += chunk.length;
        if (totalBytes > MAX_FILE_SIZE) {
          stream.destroy();
          throw new ApolloError('File exceeds maximum size of 10 MB', 'FILE_TOO_LARGE');
        }
      }

      stream.destroy();
      const readStream2 = createReadStream();
      const out = createWriteStream(`/uploads/${filename}`);
      await pipeline(readStream2, out);
      return true;
    },
  },
};
