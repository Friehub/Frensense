// [frensense]
// observation: User-controlled URL is accessed via array index before href.
// impact: Array access does not sanitize, allowing javascript: XSS.
// improvement: Validate the array element before assigning to href.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
export function UserLink({ url, label }: { url: string[]; label: string }) {
  return <a href={url[0]}>{label}</a>;
}
