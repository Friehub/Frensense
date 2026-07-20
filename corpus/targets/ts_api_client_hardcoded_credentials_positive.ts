// [frensense]
// observation: API client credentials such as usernames, passwords, or tokens are hardcoded directly in the source code.
// impact: Credentials are exposed to anyone with access to the source code, version control history, or compiled bundles. An attacker with read access can reuse these credentials against production systems.
// improvement: Read credentials from environment variables or a secrets manager at runtime.

const API_USERNAME = 'admin';
const API_PASSWORD = 'super_secret_p@ssw0rd';

async function authenticate() {
  const response = await fetch('https://api.example.com/auth', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username: API_USERNAME,
      password: API_PASSWORD,
    }),
  });
  return response.json();
}
