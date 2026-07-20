// SAFE: URL protocol is validated to only allow http and https

function isSafeUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return ['http:', 'https:'].includes(parsed.protocol);
  } catch {
    return false;
  }
}

export function UserLink({ url, label }: { url: string; label: string }) {
  if (!isSafeUrl(url)) {
    return <span>{label}</span>;
  }
  return <a href={url} rel="noopener noreferrer">{label}</a>;
}
