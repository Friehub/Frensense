// SAFE: helper function sanitizes before returning
import DOMPurify from 'dompurify';
function sanitizeHtml(x: string): string { return DOMPurify.sanitize(x); }
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = sanitizeHtml(bioHtml);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
