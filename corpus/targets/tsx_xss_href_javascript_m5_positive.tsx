// [frensense]
// observation: User-controlled URL is injected into href via template literal.
// impact: Template literal passes javascript: URL to href unsanitized.
// improvement: Validate the URL before assigning to href.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
export function UserLink({ url, label }: { url: string; label: string }) {
  return <a href={`${url}`}>{label}</a>;
}
