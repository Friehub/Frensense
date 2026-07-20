// SAFE: Uses environment variables injected at build time with restricted scope
// In Next.js, public env vars are prefixed with NEXT_PUBLIC_ — but API keys should never be public
// Instead, all API calls go through Next.js API routes (server-side)

export async function getServerSideProps(context) {
  const apiKey = process.env.MAPS_API_KEY;
  const places = await fetch(`https://maps.googleapis.com/maps/api/place/nearbysearch/json?key=${apiKey}`);
  return { props: { places: await places.json() } };
}
