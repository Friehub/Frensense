// SAFE: sanitization is applied at the final hop before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const a = bioHtml;
  const b = DOMPurify.sanitize(a);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: b }} />;
}
