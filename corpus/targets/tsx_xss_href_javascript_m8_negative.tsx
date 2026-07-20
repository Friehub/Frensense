// SAFE: array element URL is validated
function isSafeUrl(url: string): boolean {
  try { const p = new URL(url); return ['http:', 'https:'].includes(p.protocol); }
  catch { return false; }
}
export function UserLink({ url, label }: { url: string[]; label: string }) {
  if (!isSafeUrl(url[0])) return <span>{label}</span>;
  return <a href={url[0]}>{label}</a>;
}
