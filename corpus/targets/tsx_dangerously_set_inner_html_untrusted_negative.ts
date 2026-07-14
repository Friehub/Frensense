// SAFE: Input is sanitized using DOMPurify before being injected into the DOM
import DOMPurify from 'dompurify';

export function UserBio({ bioHtml }: { bioHtml: string }) {
  // SAFE: DOMPurify strips out script tags and inline handlers
  const cleanHtml = DOMPurify.sanitize(bioHtml);
  
  return (
    <div 
      className="bio-container" 
      dangerouslySetInnerHTML={{ __html: cleanHtml }} 
    />
  );
}

export function MessageItem({ message }) {
  // SAFE: standard React rendering naturally escapes text, preventing XSS
  return <span>{message.content}</span>;
}
