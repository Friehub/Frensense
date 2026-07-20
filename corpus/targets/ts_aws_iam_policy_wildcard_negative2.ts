// SAFE: IAM policy with resource-level constraints and condition keys

import { IAMClient, CreatePolicyCommand } from '@aws-sdk/client-iam';

const iam = new IAMClient({ region: 'us-east-1' });

export async function createRestrictedPolicy(policyName: string, bucketArn: string) {
  await iam.send(new CreatePolicyCommand({
    PolicyName: policyName,
    PolicyDocument: JSON.stringify({
      Version: '2012-10-17',
      Statement: [{
        Effect: 'Allow',
        Action: ['s3:GetObject'],
        Resource: `${bucketArn}/*`,
        Condition: {
          StringEquals: { 's3:prefix': ['public/'] },
        },
      }],
    }),
  }));
}
