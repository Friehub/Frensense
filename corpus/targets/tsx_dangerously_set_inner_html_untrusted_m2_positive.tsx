// [frensense]
// observation: User-controlled HTML is assigned to an intermediate variable before passing to dangerouslySetInnerHTML.
// impact: XSS via malicious HTML/script injection.
// improvement: Sanitize the intermediate variable with DOMPurify before injecting.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = bioHtml;
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
