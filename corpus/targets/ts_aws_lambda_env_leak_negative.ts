// SAFE: Only log non-sensitive configuration metadata

import { Handler } from 'aws-lambda';

export const handler: Handler = async (event) => {
  console.log('Lambda invoked with event type:', typeof event);
  console.log('Runtime:', process.env.AWS_EXECUTION_ENV);
  console.log('Region:', process.env.AWS_REGION);

  return {
    statusCode: 200,
    body: JSON.stringify({ message: 'Processing complete' }),
  };
};
