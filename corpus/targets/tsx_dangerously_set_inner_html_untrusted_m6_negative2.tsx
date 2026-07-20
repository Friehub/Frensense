// SAFE: React JSX renders the concatenated value escaped
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container">{"<div class='card'>" + bioHtml + "</div>"}</div>;
}
