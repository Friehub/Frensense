// [frensense]
// observation: IAM policy uses `Action: *` and `Resource: *`, granting unrestricted access to all AWS services and resources.
// impact: A compromised principal with this policy gains full administrative access to the entire AWS account.
// improvement: Scope IAM policies to the minimum required actions and specific resource ARNs.

import { IAMClient, CreatePolicyCommand } from '@aws-sdk/client-iam';

const iam = new IAMClient({ region: 'us-east-1' });

export async function createAdminPolicy(policyName: string) {
  await iam.send(new CreatePolicyCommand({
    PolicyName: policyName,
    PolicyDocument: JSON.stringify({
      Version: '2012-10-17',
      Statement: [{
        Effect: 'Allow',
        Action: '*',
        Resource: '*',
      }],
    }),
  }));
}
