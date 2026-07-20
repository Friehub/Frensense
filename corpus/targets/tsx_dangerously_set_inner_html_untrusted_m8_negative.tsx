// SAFE: array element is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string[] }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(bioHtml[0]) }} />;
}
