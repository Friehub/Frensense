// SAFE: API key transmitted via Authorization header, not in URL
export async function fetchUserData(userId: string): Promise<Response> {
  return fetch(`https://api.example.com/users/${userId}`, {
    headers: { Authorization: `Bearer ${API_KEY}` }
  });
}

export async function searchItems(query: string): Promise<Response> {
  return fetch(`https://api.example.com/search?q=${encodeURIComponent(query)}`, {
    headers: { Authorization: `Bearer ${API_KEY}` }
  });
}
