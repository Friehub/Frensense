// [frensense]
// observation: User input flows through two assignments before reaching dangerouslySetInnerHTML.
// impact: XSS via multi-hop taint propagation through intermediate variables.
// improvement: Sanitize at any point in the chain before DOM injection.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const a = bioHtml;
  const b = a;
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: b }} />;
}
