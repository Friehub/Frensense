// SAFE: React standard rendering is used instead of dangerouslySetInnerHTML
function passthrough(x: string): string { return x; }
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = passthrough(bioHtml);
  return <div className="bio-container">{html}</div>;
}
