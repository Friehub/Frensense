// SAFE: Pre-signed URL uses a configurable expiration that defaults to a short window

const DEFAULT_URL_EXPIRY = 300;

import { S3Client, GetObjectCommand } from '@aws-sdk/client-s3';
import { getSignedUrl } from '@aws-sdk/s3-request-presigner';

const s3 = new S3Client({});

export async function generateDownloadUrl(key: string, expiresIn = DEFAULT_URL_EXPIRY): Promise<string> {
  const command = new GetObjectCommand({ Bucket: 'my-bucket', Key: key });
  return getSignedUrl(s3, command, { expiresIn: Math.min(expiresIn, 86400) });
}
