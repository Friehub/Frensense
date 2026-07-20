// [frensense]
// observation: An API key or service credential is embedded directly in frontend JavaScript code that is sent to the browser.
// impact: Anyone can view the page source, decompile the bundle, or inspect network requests to extract the API key and use it to call the service directly.
// improvement: Never embed secrets in client-side code. Route API calls through a backend proxy that holds the secret server-side.

const GOOGLE_MAPS_KEY = 'AIzaSyD...';
const SENTRY_DSN = 'https://abc@def.ingest.sentry.io/123';
const STRIPE_PUBLISHABLE_KEY = 'pk_live_abc123';

export async function loadMap(): Promise<void> {
  const script = document.createElement('script');
  script.src = `https://maps.googleapis.com/maps/api/js?key=${GOOGLE_MAPS_KEY}`;
  document.head.appendChild(script);
}
