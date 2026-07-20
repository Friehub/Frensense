// SAFE: template literal URL is sanitized
function sanitizeUrl(url: string): string | null {
  try { const p = new URL(url); return ['http:', 'https:', 'mailto:'].includes(p.protocol) ? url : null; }
  catch { return url.startsWith('/') || url.startsWith('#') ? url : null; }
}
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = sanitizeUrl(`${url}`);
  if (!u) return <span>{label}</span>;
  return <a href={u} target="_blank" rel="noopener noreferrer">{label}</a>;
}
