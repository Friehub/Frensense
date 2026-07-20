// SAFE: API key transmitted via a custom header (X-Api-Key)
export async function fetchUserData(userId: string): Promise<Response> {
  return fetch(`https://api.example.com/users/${userId}`, {
    headers: { 'X-Api-Key': API_KEY }
  });
}

export async function searchItems(query: string): Promise<Response> {
  return fetch(`https://api.example.com/search?q=${encodeURIComponent(query)}`, {
    headers: { 'X-Api-Key': API_KEY }
  });
}
