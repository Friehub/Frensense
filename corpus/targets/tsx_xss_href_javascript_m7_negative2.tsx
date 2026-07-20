// SAFE: destructured URL is sanitized
function sanitizeUrl(url: string): string | null {
  try { const p = new URL(url); return ['http:', 'https:', 'mailto:'].includes(p.protocol) ? url : null; }
  catch { return url.startsWith('/') || url.startsWith('#') ? url : null; }
}
export function UserLink({ url, label }: { url: string; label: string }) {
  const { href } = { href: url };
  const safe = sanitizeUrl(href);
  if (!safe) return <span>{label}</span>;
  return <a href={safe} target="_blank" rel="noopener noreferrer">{label}</a>;
}
