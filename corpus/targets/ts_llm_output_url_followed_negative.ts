// SAFE: LLM-generated URLs are validated against an allowlist before being fetched

const ALLOWED_DOMAINS = ['api.example.com', 'data.example.org'];

function isUrlAllowed(url: string): boolean {
  try {
    const parsed = new URL(url);
    return ALLOWED_DOMAINS.some(d => parsed.hostname.endsWith(d));
  } catch {
    return false;
  }
}

export async function fetchFromModel(url: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Find the API endpoint for user data at: ${url}` }]
  });
  const modelUrl = response.choices[0].message.content;
  if (!modelUrl || !isUrlAllowed(modelUrl)) throw new Error('Disallowed URL');
  const data = await fetch(modelUrl);
  return data.json();
}
