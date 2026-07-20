// SAFE: template literal result is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = `${bioHtml}`;
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html) }} />;
}
