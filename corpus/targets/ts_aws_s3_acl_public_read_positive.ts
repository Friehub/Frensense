// [frensense]
// observation: S3 bucket ACL is set to public-read, granting anonymous users read access to all objects.
// impact: Any object in the bucket is readable by the public without authentication, potentially exposing sensitive data.
// improvement: Use private ACL and grant access only through IAM policies or pre-signed URLs.

import { S3Client, PutBucketAclCommand } from '@aws-sdk/client-s3';

const s3 = new S3Client({ region: 'us-east-1' });

export async function configureBucket(bucketName: string) {
  await s3.send(new PutBucketAclCommand({
    Bucket: bucketName,
    ACL: 'public-read',
  }));
}
