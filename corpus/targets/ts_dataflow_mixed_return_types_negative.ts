// SAFE: caller checks for Error before treating the result as the expected type

async function fetchUserData(userId: string): Promise<{ name: string } | Error> {
  const response = await fetch(`/api/users/${userId}`);
  if (!response.ok) return new Error('Failed to fetch');
  return response.json();
}

async function displayUser(userId: string) {
  const result = await fetchUserData(userId);
  if (result instanceof Error) {
    return { name: 'Unknown', error: result.message };
  }
  return { name: result.name.toUpperCase() };
}
