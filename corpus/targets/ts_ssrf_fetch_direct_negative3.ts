// SAFE: validates host against an allowlist before fetching
const ALLOWED_HOSTS = ['api.example.com', 'data.example.com'];

function isAllowedHost(url: string): boolean {
  for (const host of ALLOWED_HOSTS) {
    if (url.startsWith(`https://${host}/`)) {
      return true;
    }
  }
  return false;
}

export async function fetchData(url: string): Promise<unknown> {
  if (!isAllowedHost(url)) {
    throw new Error('Host not allowed');
  }
  const response = await fetch(url);
  return await response.json();
}
