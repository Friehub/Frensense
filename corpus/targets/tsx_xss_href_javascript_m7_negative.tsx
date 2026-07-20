// SAFE: destructured URL is validated
function isSafeUrl(url: string): boolean {
  try { const p = new URL(url); return ['http:', 'https:'].includes(p.protocol); }
  catch { return false; }
}
export function UserLink({ url, label }: { url: string; label: string }) {
  const { href } = { href: url };
  if (!isSafeUrl(href)) return <span>{label}</span>;
  return <a href={href}>{label}</a>;
}
