// SAFE: IAM policy scoped to specific actions and resources

import { IAMClient, CreatePolicyCommand } from '@aws-sdk/client-iam';

const iam = new IAMClient({ region: 'us-east-1' });

export async function createS3ReadPolicy(policyName: string, bucketArn: string) {
  await iam.send(new CreatePolicyCommand({
    PolicyName: policyName,
    PolicyDocument: JSON.stringify({
      Version: '2012-10-17',
      Statement: [{
        Effect: 'Allow',
        Action: ['s3:GetObject', 's3:ListBucket'],
        Resource: [bucketArn, `${bucketArn}/*`],
      }],
    }),
  }));
}
