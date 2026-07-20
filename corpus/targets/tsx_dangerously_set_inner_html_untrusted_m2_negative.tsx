// SAFE: intermediate variable is sanitized with DOMPurify before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = DOMPurify.sanitize(bioHtml);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
