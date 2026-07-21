// SAFE: validates image URL against a domain allowlist before rendering
const ALLOWED_DOMAINS = ['storage.example.com', 'cdn.example.com'];

function isAllowedImageUrl(src: string): boolean {
  for (const domain of ALLOWED_DOMAINS) {
    if (src.startsWith(`https://${domain}/`)) {
      return true;
    }
  }
  return false;
}

export function Avatar({ url }: { url: string }) {
  if (!isAllowedImageUrl(url)) {
    return <img src="/fallback.png" alt="fallback" />;
  }
  return <img src={url} alt="avatar" className="avatar" />;
}
