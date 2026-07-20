// [frensense]
// observation: A pre-signed URL for S3 or R2 is generated with no expiration or an excessively long expiration time.
// impact: Anyone who obtains the URL can access or upload files indefinitely, leading to data exposure or bucket fill-up.
// improvement: Always set a short expiration time (e.g., 5 minutes) for pre-signed URLs.

import { S3Client, GetObjectCommand } from '@aws-sdk/client-s3';
import { getSignedUrl } from '@aws-sdk/s3-request-presigner';

const s3 = new S3Client({});

export async function generateDownloadUrl(key: string): Promise<string> {
  const command = new GetObjectCommand({ Bucket: 'my-bucket', Key: key });
  return getSignedUrl(s3, command);
}
