// SAFE: Secrets are loaded server-side and used only in API routes, never exposed to the client bundle

const INTERNAL_API_KEY = process.env.INTERNAL_API_KEY;

export async function getServerSideProps() {
  return {
    props: {}
  };
}

export async function apiProxy(endpoint: string) {
  const response = await fetch(`https://internal-api.example.com/${endpoint}`, {
    headers: { 'Authorization': `Bearer ${INTERNAL_API_KEY}` }
  });
  return response.json();
}
