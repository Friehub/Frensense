// SAFE: destructured value is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const { content } = { content: DOMPurify.sanitize(bioHtml) };
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: content }} />;
}
