// SAFE: React JSX escapes the template literal content
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container">{`${bioHtml}`}</div>;
}
