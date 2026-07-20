// SAFE: Uses a sanitization helper that strips dangerous HTML before rendering
import DOMPurify from 'dompurify';

const SANITIZE_OPTIONS = { ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a'], ALLOWED_ATTR: ['href'] };

export function UserBio({ bioHtml }: { bioHtml: string }) {
  const cleanHtml = DOMPurify.sanitize(bioHtml, SANITIZE_OPTIONS);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: cleanHtml }} />;
}

export function SafeMessage({ content }: { content: string }) {
  return <span>{content}</span>;
}
