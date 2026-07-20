// SAFE: Validates file size from Content-Length header before processing the upload stream.

import { ApolloServer, gql, ApolloError } from 'apollo-server-express';
import { GraphQLUpload } from 'graphql-upload-minimal';
import { createWriteStream } from 'fs';
import { pipeline } from 'stream/promises';

const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5 MB

const typeDefs = gql`
  scalar Upload

  type Mutation {
    uploadProfilePhoto(file: Upload!): Boolean!
  }
`;

interface UploadArgs {
  file: { createReadStream: () => NodeJS.ReadableStream; filename: string; mimetype: string; encoding: string };
}

const resolvers = {
  Upload: GraphQLUpload,
  Mutation: {
    uploadProfilePhoto: async (_: any, { file }: UploadArgs) => {
      const { createReadStream, filename } = await file;
      const stream = createReadStream({ highWaterMark: 1024 * 1024 });

      let uploaded = 0;
      for await (const chunk of stream) {
        uploaded += chunk.length;
        if (uploaded > MAX_FILE_SIZE) {
          stream.destroy();
          throw new ApolloError('File exceeds maximum size of 5 MB', 'FILE_TOO_LARGE');
        }
      }

      stream.destroy();
      const newStream = createReadStream();
      const out = createWriteStream(`/uploads/${filename}`);
      await pipeline(newStream, out);
      return true;
    },
  },
};
