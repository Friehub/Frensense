// SAFE: URL is sanitized by stripping dangerous protocols and ensuring the scheme is safe

function sanitizeUrl(url: string): string | null {
  const allowedProtocols = ['http:', 'https:', 'mailto:'];
  try {
    const parsed = new URL(url);
    if (allowedProtocols.includes(parsed.protocol)) {
      return url;
    }
    return null;
  } catch {
    return url.startsWith('/') || url.startsWith('#') ? url : null;
  }
}

export function UserLink({ url, label }: { url: string; label: string }) {
  const safeUrl = sanitizeUrl(url);
  if (!safeUrl) {
    return <span>{label}</span>;
  }
  return <a href={safeUrl} target="_blank" rel="noopener noreferrer">{label}</a>;
}
