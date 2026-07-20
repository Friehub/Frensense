// SAFE: Pre-signed URL is generated with a 5-minute expiration time

import { S3Client, GetObjectCommand } from '@aws-sdk/client-s3';
import { getSignedUrl } from '@aws-sdk/s3-request-presigner';

const s3 = new S3Client({});

export async function generateDownloadUrl(key: string): Promise<string> {
  const command = new GetObjectCommand({ Bucket: 'my-bucket', Key: key });
  return getSignedUrl(s3, command, { expiresIn: 300 });
}
