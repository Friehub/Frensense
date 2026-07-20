// SAFE: Store a short-lived access token in memory and a refresh token in an httpOnly cookie.

let inMemoryToken: string | null = null;

async function login(): Promise<void> {
  const response = await fetch('/api/auth/login', {
    method: 'POST',
    credentials: 'include',
  });
  const { accessToken } = await response.json() as { accessToken: string };
  inMemoryToken = accessToken;
}

function getAccessToken(): string | null {
  return inMemoryToken;
}

async function fetchWithAuth(url: string): Promise<Response> {
  const token = getAccessToken();
  const headers: Record<string, string> = {};
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }
  const response = await fetch(url, {
    headers,
    credentials: 'include',
  });
  if (response.status === 401) {
    const refreshResponse = await fetch('/api/auth/refresh', {
      method: 'POST',
      credentials: 'include',
    });
    if (refreshResponse.ok) {
      const { accessToken } = await refreshResponse.json() as { accessToken: string };
      inMemoryToken = accessToken;
      headers.Authorization = `Bearer ${accessToken}`;
      return fetch(url, { headers, credentials: 'include' });
    }
  }
  return response;
}
