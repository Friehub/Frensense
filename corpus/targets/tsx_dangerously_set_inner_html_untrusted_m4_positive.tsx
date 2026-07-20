// [frensense]
// observation: User input passes through a helper function that does not sanitize before reaching dangerouslySetInnerHTML.
// impact: XSS via unsanitized helper return value.
// improvement: Ensure the helper sanitizes its return value.
function passthrough(x: string): string { return x; }
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = passthrough(bioHtml);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
