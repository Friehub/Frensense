// [frensense]
// observation: Lambda function logs environment variables containing secrets, exposing them in CloudWatch logs.
// impact: API keys, database credentials, and other secrets become visible to anyone with CloudWatch log access.
// improvement: Never log environment variables or sensitive configuration values directly.

import { Handler } from 'aws-lambda';

export const handler: Handler = async (event) => {
  console.log('Environment:', process.env);
  console.log('DB_URL:', process.env.DATABASE_URL);
  console.log('API_KEY:', process.env.API_KEY);

  return {
    statusCode: 200,
    body: JSON.stringify({ message: 'Processing complete' }),
  };
};
