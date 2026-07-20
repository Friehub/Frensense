// [frensense]
// observation: React components pass untrusted or unvalidated props directly into dangerouslySetInnerHTML.
// impact: Execution of arbitrary JavaScript in the victim's browser (Cross-Site Scripting, XSS) via malicious HTML payloads.
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering instead.

export function UserBio({ bioHtml }: { bioHtml: string }) {
  // VULNERABLE: user-controlled string rendered as HTML without sanitization
  return (
    <div 
      className="bio-container" 
      dangerouslySetInnerHTML={{ __html: bioHtml }} 
    />
  );
}

export function MessageItem({ message }) {
  // VULNERABLE: message.content might contain malicious script tags
  return <span dangerouslySetInnerHTML={{ __html: message.content }} />;
}
