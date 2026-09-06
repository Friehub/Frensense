// SAFE: helper result is validated before rendering
function isSafeUrl(url: string): boolean {
  try { const p = new URL(url); return ['http:', 'https:'].includes(p.protocol); }
  catch { return false; }
}
function processUrl(x: string): string { return x; }
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = processUrl(url);
  if (!isSafeUrl(u)) return <span>{label}</span>;
  return <a href={u}>{label}</a>;
}
