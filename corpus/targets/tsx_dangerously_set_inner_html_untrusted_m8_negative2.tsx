// SAFE: React JSX renders the array element escaped
export function UserBio({ bioHtml }: { bioHtml: string[] }) {
  return <div className="bio-container">{bioHtml[0]}</div>;
}
