// [frensense]
// observation: User-controlled URL is destructured before being set as href.
// impact: Destructuring passes the unsanitized URL to href.
// improvement: Validate after destructuring.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
export function UserLink({ url, label }: { url: string; label: string }) {
  const { href } = { href: url };
  return <a href={href}>{label}</a>;
}
