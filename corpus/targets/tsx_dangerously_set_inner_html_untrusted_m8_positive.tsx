// [frensense]
// observation: User input is accessed via array index before passing to dangerouslySetInnerHTML.
// impact: XSS — array access does not sanitize the value.
// improvement: Sanitize the array element before injection.
export function UserBio({ bioHtml }: { bioHtml: string[] }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: bioHtml[0] }} />;
}
