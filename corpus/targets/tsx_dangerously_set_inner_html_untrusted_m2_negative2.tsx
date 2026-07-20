// SAFE: React standard rendering escapes the intermediate variable content
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = bioHtml;
  return <div className="bio-container">{html}</div>;
}
