// [frensense]
// observation: The API key is transmitted as a query parameter in the URL (e.g., ?key=abc123 or ?apiKey=xyz).
// impact: API keys leak via server logs, browser history, Referer headers, and are visible in the address bar. Any third party with log access can steal the key.
// improvement: Transmit API keys exclusively via the Authorization header (Bearer token) or a dedicated custom header.

export async function fetchUserData(userId: string): Promise<Response> {
  return fetch(`https://api.example.com/users/${userId}?key=${API_KEY}`);
}

export async function searchItems(query: string): Promise<Response> {
  return fetch(`https://api.example.com/search?q=${query}&apikey=${API_KEY}`);
}
