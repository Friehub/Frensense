// SAFE: Uses safe rendering without dangerouslySetInnerHTML
import DOMPurify from "dompurify";
const SANITIZE_OPTIONS = { ALLOWED_TAGS: ["b", "i", "em", "strong", "a"], ALLOWED_ATTR: ["href"] };
export function ComponentA({ bioHtml }: { bioHtml: string }) {
  const cleanHtml = DOMPurify.sanitize(bioHtml, SANITIZE_OPTIONS);
  return <div className="container" dangerouslySetInnerHTML={{ __html: cleanHtml }} />;
}
export function ComponentB({ content }: { content: string }) {
  return <span>{content}</span>;
}
