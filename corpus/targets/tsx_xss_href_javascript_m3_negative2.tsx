// SAFE: multi-hop URL is sanitized before rendering
function sanitizeUrl(url: string): string | null {
  try { const p = new URL(url); return ['http:', 'https:', 'mailto:'].includes(p.protocol) ? url : null; }
  catch { return url.startsWith('/') || url.startsWith('#') ? url : null; }
}
export function UserLink({ url, label }: { url: string; label: string }) {
  const a = url;
  const b = sanitizeUrl(a);
  if (!b) return <span>{label}</span>;
  return <a href={b} target="_blank" rel="noopener noreferrer">{label}</a>;
}
