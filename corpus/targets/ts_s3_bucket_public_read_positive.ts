// [frensense]
// observation: An S3 or R2 bucket ACL is set to public-read or public-read-write, allowing unauthenticated access.
// impact: Anyone on the internet can read (or write) all objects in the bucket, leading to data exposure.
// improvement: Never use public ACLs. Use IAM policies, pre-signed URLs, or CloudFront signed URLs instead.

import { S3Client, PutBucketAclCommand } from '@aws-sdk/client-s3';

const s3 = new S3Client({});

export async function createPublicBucket(name: string) {
  await s3.send(new PutBucketAclCommand({
    Bucket: name,
    ACL: 'public-read'
  }));
}
