// SAFE: concatenated result is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = "<div class='card'>" + bioHtml + "</div>";
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html) }} />;
}
