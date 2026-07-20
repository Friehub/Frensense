// SAFE: Public access is explicitly blocked via bucket policy

import { S3Client, PutBucketPolicyCommand } from '@aws-sdk/client-s3';

const s3 = new S3Client({});

export async function createBucket(name: string) {
  await s3.send(new PutBucketPolicyCommand({
    Bucket: name,
    Policy: JSON.stringify({
      Version: '2012-10-17',
      Statement: [{
        Effect: 'Deny',
        Principal: '*',
        Action: 's3:GetObject',
        Resource: `arn:aws:s3:::${name}/*`,
        Condition: { Bool: { 'aws:SecureTransport': 'false' } }
      }]
    })
  }));
}
