// [frensense]
// observation: User-controlled URL passes through a helper function that does not validate protocols before href.
// impact: Helper returns dangerous javascript: URL directly to href.
// improvement: Add protocol validation in the helper function.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
function processUrl(x: string): string { return x; }
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = processUrl(url);
  return <a href={u}>{label}</a>;
}
