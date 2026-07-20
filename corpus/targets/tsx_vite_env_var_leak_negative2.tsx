// SAFE: Only exposes non-sensitive env vars prefixed with PUBLIC_APP_ via Vite
export function AppVersion() {
  return (
    <div>
      App version: {import.meta.env.PUBLIC_APP_VERSION}
    </div>
  );
}
