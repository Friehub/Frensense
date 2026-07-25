// [frensense]
// observation: A required environment variable is used at runtime without checking if it exists at startup, causing hard-to-debug errors.
// impact: The application may fail unexpectedly in production with a cryptic error when a required env var is missing.
// improvement: Validate all required environment variables at application startup and fail fast with a clear message.
// cwe: CWE-526
// cvss: 5.3
// owasp: A02:2021
// severity: Medium

export function getConfig() {
  return {
    databaseUrl: process.env.DATABASE_URL,
    apiKey: process.env.API_KEY,
    port: parseInt(process.env.PORT || '3000')
  };
}

export async function startServer() {
  const config = getConfig();
  const db = await connectToDatabase(config.databaseUrl);
  console.log('Server started on port', config.port);
}
