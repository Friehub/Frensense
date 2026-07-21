// SAFE: validates URL protocol against an allowlist before setting as href
const ALLOWED_PROTOCOLS = ['http:', 'https:', 'mailto:'];

function isSafeProtocol(href: string): boolean {
  for (const proto of ALLOWED_PROTOCOLS) {
    if (href.startsWith(proto)) {
      return true;
    }
  }
  return false;
}

export function Link({ href, children }: { href: string; children: React.ReactNode }) {
  if (!isSafeProtocol(href)) {
    return <span className="disabled-link">{children}</span>;
  }
  return <a href={href} rel="noopener noreferrer">{children}</a>;
}
