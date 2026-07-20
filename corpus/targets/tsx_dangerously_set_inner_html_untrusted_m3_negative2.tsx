// SAFE: React JSX rendering escapes the multi-hop value
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const a = bioHtml;
  const b = a;
  return <div className="bio-container">{b}</div>;
}
