// SAFE: multi-hop URL is validated before rendering
function isSafeUrl(url: string): boolean {
  try { const p = new URL(url); return ['http:', 'https:'].includes(p.protocol); }
  catch { return false; }
}
export function UserLink({ url, label }: { url: string; label: string }) {
  const a = url;
  const b = a;
  if (!isSafeUrl(b)) return <span>{label}</span>;
  return <a href={b}>{label}</a>;
}
