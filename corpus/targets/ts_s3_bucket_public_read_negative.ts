// SAFE: Bucket ACLs are not set — access is controlled via IAM policies and pre-signed URLs

import { S3Client, CreateBucketCommand } from '@aws-sdk/client-s3';

const s3 = new S3Client({});

export async function createPrivateBucket(name: string) {
  await s3.send(new CreateBucketCommand({
    Bucket: name
  }));
}
