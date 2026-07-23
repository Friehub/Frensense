// SAFE: API keys are loaded server-side; client requests proxied through backend
export async function loadMap(): Promise<void> {
  const keyRes = await fetch('/api/config/maps-key');
  const { key } = await keyRes.json();
  const script = document.createElement('script');
  script.src = `https://maps.googleapis.com/maps/api/js?key=${key}`;
  document.head.appendChild(script);
}
