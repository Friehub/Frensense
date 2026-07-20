// SAFE: credentials are read from environment variables at runtime

async function authenticate() {
  const response = await fetch('https://api.example.com/auth', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username: process.env.API_USERNAME,
      password: process.env.API_PASSWORD,
    }),
  });
  return response.json();
}
