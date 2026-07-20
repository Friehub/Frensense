// SAFE: helper function itself validates the URL
function sanitizeUrl(url: string): string | null {
  try { const p = new URL(url); return ['http:', 'https:', 'mailto:'].includes(p.protocol) ? url : null; }
  catch { return url.startsWith('/') || url.startsWith('#') ? url : null; }
}
function processUrl(x: string): string | null { return sanitizeUrl(x); }
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = processUrl(url);
  if (!u) return <span>{label}</span>;
  return <a href={u} target="_blank" rel="noopener noreferrer">{label}</a>;
}
