// SAFE: React JSX renders the destructured value escaped
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const { content } = { content: bioHtml };
  return <div className="bio-container">{content}</div>;
}
