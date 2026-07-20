// SAFE: Sanitizes user input before passing to meta tags using HTML escaping
import type { MetaFunction } from "@remix-run/node";

function escapeMeta(str: string): string {
  return str.replace(/"/g, "&quot;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

export const meta: MetaFunction = ({ data }) => {
  const safeTitle = escapeMeta(data?.userProvidedTitle ?? "");
  const safeDesc = escapeMeta(data?.userInput ?? "");
  return [
    { title: safeTitle },
    { name: "description", content: safeDesc },
  ];
};
