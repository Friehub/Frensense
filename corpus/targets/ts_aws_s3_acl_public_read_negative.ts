// SAFE: Bucket ACL set to private

import { S3Client, PutBucketAclCommand } from '@aws-sdk/client-s3';

const s3 = new S3Client({ region: 'us-east-1' });

export async function configureBucket(bucketName: string) {
  await s3.send(new PutBucketAclCommand({
    Bucket: bucketName,
    ACL: 'private',
  }));
}
