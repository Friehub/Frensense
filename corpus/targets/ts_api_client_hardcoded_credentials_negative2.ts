// SAFE: uses a dedicated secrets manager to fetch credentials at runtime

interface SecretsProvider {
  get(key: string): Promise<string>;
}

async function authenticate(secrets: SecretsProvider) {
  const [username, password] = await Promise.all([
    secrets.get('api/username'),
    secrets.get('api/password'),
  ]);
  const response = await fetch('https://api.example.com/auth', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password }),
  });
  return response.json();
}
