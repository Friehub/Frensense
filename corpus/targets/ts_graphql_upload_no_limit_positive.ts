// [frensense]
// observation: A GraphQL upload mutation accepts file data without enforcing any size limit, allowing clients to upload arbitrarily large files.
// impact: An attacker can upload multi-gigabyte files, filling disk space, exhausting memory during processing, and causing denial of service.
// improvement: Enforce a maximum file size in the upload resolver before processing the file payload.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import { ApolloServer, gql } from 'apollo-server-express';
import { GraphQLUpload } from 'graphql-upload-minimal';
import { createWriteStream } from 'fs';
import { pipeline } from 'stream/promises';

const typeDefs = gql`
  scalar Upload

  type Mutation {
    uploadProfilePhoto(file: Upload!): Boolean!
    uploadDocument(file: Upload!): String!
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
      const out = createWriteStream(`/uploads/${filename}`);
      await pipeline(stream, out);
      return true;
    },
    uploadDocument: async (_: any, { file }: UploadArgs) => {
      const { createReadStream, filename } = await file;
      const stream = createReadStream();
      const chunks: Buffer[] = [];
      for await (const chunk of stream) {
        chunks.push(chunk);
      }
      const fullBuffer = Buffer.concat(chunks);
      await storeDocument(filename, fullBuffer);
      return filename;
    },
  },
};
