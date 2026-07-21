// [frensense]
// observation: React components pass untrusted or unvalidated props directly into dangerouslySetInnerHTML.
// impact: Execution of arbitrary JavaScript in the victim's browser (Cross-Site Scripting, XSS) via malicious HTML payloads.
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering instead.
import { useState } from 'react';

export function UserBio() {
  const [bioHtml] = useState('<img src=x onerror=alert(1)>');
  return (
    <div className="bio-container" dangerouslySetInnerHTML={{ __html: bioHtml }} />
  );
}
