// SAFE: Uses DOMPurify library for HTML, robust regex for filenames and URLs
import DOMPurify from "dompurify";

function sanitizeHtml(input: string): string {
  return DOMPurify.sanitize(input, { ALLOWED_TAGS: ["b", "i", "em", "strong", "a"], ALLOWED_ATTR: ["href"] });
}

function sanitizeFilename(input: string): string {
  return input.replace(/[^a-zA-Z0-9._-]/g, "_").replace(/^\.+/, "").slice(0, 255) || "unnamed_file";
}

function sanitizeUrl(input: string): string {
  try {
    const url = new URL(input);
    if (!["http:", "https:"].includes(url.protocol)) return "";
    return url.toString();
  } catch {
    return "";
  }
}
