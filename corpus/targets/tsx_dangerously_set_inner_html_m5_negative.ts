// SAFE: Implements safe alternative
// SAFE: Input is sanitized using DOMPurify before being injected into the DOM
import DOMPurify from "dompurify";
export function ComponentA({ bioHtml }: { bioHtml: string }) {
  const cleanHtml = DOMPurify.sanitize(bioHtml);
  return <div className="container" dangerouslySetInnerHTML={{ __html: cleanHtml }} />;
}
export function ComponentB({ message }: { message: { content: string } }) {
  return <span>{message.content}</span>;
}
