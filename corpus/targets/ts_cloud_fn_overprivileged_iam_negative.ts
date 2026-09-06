// SAFE: The Lambda function uses a minimal IAM role with only the specific permissions required

import { LambdaClient, CreateFunctionCommand } from '@aws-sdk/client-lambda';

const lambda = new LambdaClient({});

export async function deployFunction() {
  await lambda.send(new CreateFunctionCommand({
    FunctionName: 'data-processor',
    Role: 'arn:aws:iam::123456789012:role/DataProcessorRole',
    Runtime: 'nodejs18.x',
    Handler: 'index.handler',
    Code: { ZipFile: Buffer.from('...') }
  }));
}
