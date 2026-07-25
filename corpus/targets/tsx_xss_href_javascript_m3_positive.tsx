// [frensense]
// observation: User-controlled URL flows through two assignments before reaching href.
// impact: Multi-hop taint propagation enables javascript: XSS.
// improvement: Validate the URL at any point before assigning to href.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
export function UserLink({ url, label }: { url: string; label: string }) {
  const a = url;
  const b = a;
  return <a href={b}>{label}</a>;
}
