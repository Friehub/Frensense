// [frensense]
// observation: A cloud function (Lambda, Cloud Function) is configured with an overly permissive IAM role like AdministratorAccess.
// impact: If the function is compromised, an attacker gains full control over the entire cloud account.
// improvement: Follow the principle of least privilege — grant only the specific permissions the function needs.

import { LambdaClient, CreateFunctionCommand } from '@aws-sdk/client-lambda';

const lambda = new LambdaClient({});

export async function deployFunction() {
  await lambda.send(new CreateFunctionCommand({
    FunctionName: 'data-processor',
    Role: 'arn:aws:iam::123456789012:role/AdminRole',
    Runtime: 'nodejs18.x',
    Handler: 'index.handler',
    Code: { ZipFile: Buffer.from('...') }
  }));
}
