// [frensense]
// observation: User input is concatenated into an HTML string passed to dangerouslySetInnerHTML.
// impact: XSS — string concatenation does not sanitize embedded HTML/script.
// improvement: Sanitize the concatenated result or avoid dangerouslySetInnerHTML.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = "<div class='card'>" + bioHtml + "</div>";
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
