// [frensense]
// observation: User input is injected via template literal into dangerouslySetInnerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize the template literal output or avoid dangerouslySetInnerHTML.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: `${bioHtml}` }} />;
}
