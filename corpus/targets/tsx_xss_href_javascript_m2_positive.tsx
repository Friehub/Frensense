// [frensense]
// observation: User-controlled URL is assigned to an intermediate variable before being set as href.
// impact: Clicking the link can execute javascript: XSS.
// improvement: Validate the URL protocol before assigning it to href.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = url;
  return <a href={u}>{label}</a>;
}
