// SAFE: IAM permissions are scoped to specific resource ARNs and actions

import { LambdaClient, CreateFunctionCommand } from '@aws-sdk/client-lambda';

const lambda = new LambdaClient({});

export async function deployFunction() {
  await lambda.send(new CreateFunctionCommand({
    FunctionName: 'data-processor',
    Role: 'arn:aws:iam::123456789012:role/DataProcessorRole',
    Runtime: 'nodejs18.x',
    Handler: 'index.handler',
    Code: { ZipFile: Buffer.from('...') },
    Environment: {
      Variables: {
        BUCKET_NAME: 'my-processed-data',
        TABLE_NAME: 'my-db-table'
      }
    }
  }));
}
