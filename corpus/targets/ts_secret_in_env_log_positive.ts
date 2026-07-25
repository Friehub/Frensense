// [frensense]
// observation: A secret or API key stored in an environment variable is logged at application startup, exposing it in logs.
// impact: Anyone with access to log files or log aggregation services can read the secret, compromising the infrastructure.
// improvement: Never log environment variables that contain secrets. Use a blocklist to filter them out.
// cwe: CWE-526
// cvss: 5.3
// owasp: A02:2021
// severity: Medium

export function startApp() {
  console.log('Starting app with config:');
  console.log('DATABASE_URL:', process.env.DATABASE_URL);
  console.log('API_KEY:', process.env.API_KEY);
  console.log('REDIS_HOST:', process.env.REDIS_HOST);
  // start the server...
}
