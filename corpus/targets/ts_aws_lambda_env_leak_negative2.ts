// SAFE: Secrets retrieved from Secrets Manager, logged with redaction

import { Handler } from 'aws-lambda';
import { SecretsManagerClient, GetSecretValueCommand } from '@aws-sdk/client-secrets-manager';

const secretsManager = new SecretsManagerClient({ region: 'us-east-1' });

function redact(value: string): string {
  return value.length > 4
    ? `${value.substring(0, 4)}...${value.substring(value.length - 1)}`
    : '****';
}

export const handler: Handler = async (event) => {
  const secret = await secretsManager.send(new GetSecretValueCommand({
    SecretId: 'prod/db-url',
  }));

  const dbUrl = secret.SecretString!;
  console.log('Database configured (redacted):', redact(dbUrl));

  return {
    statusCode: 200,
    body: JSON.stringify({ message: 'Processing complete' }),
  };
};
