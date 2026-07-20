// [frensense]
// observation: User input is destructured before being passed to dangerouslySetInnerHTML.
// impact: XSS — destructuring does not sanitize the extracted value.
// improvement: Sanitize after destructuring or avoid dangerouslySetInnerHTML.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const { content } = { content: bioHtml };
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: content }} />;
}
