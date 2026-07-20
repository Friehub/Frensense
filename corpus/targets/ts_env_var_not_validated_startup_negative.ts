// SAFE: Required environment variables are validated at startup with clear error messages

const REQUIRED_VARS = ['DATABASE_URL', 'API_KEY', 'PORT'] as const;

function validateEnv(): void {
  for (const varName of REQUIRED_VARS) {
    if (!process.env[varName]) {
      throw new Error(`Missing required environment variable: ${varName}`);
    }
  }
}

export function getConfig() {
  validateEnv();
  return {
    databaseUrl: process.env.DATABASE_URL!,
    apiKey: process.env.API_KEY!,
    port: parseInt(process.env.PORT!, 10)
  };
}
