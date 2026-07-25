// [frensense]
// observation: User input is injected via template literal into dangerouslySetInnerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize the template literal output or avoid dangerouslySetInnerHTML.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: `${bioHtml}` }} />;
}
