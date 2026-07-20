// [frensense]
// observation: Remix meta function returns user input directly without sanitization, enabling XSS injection into <head> metadata.
// impact: Attacker-controlled content rendered in meta tags can execute scripts via document.title or meta description injection, leading to XSS.
// improvement: Sanitize user input before returning from meta function, or use encodeURIComponent for URL-based values.

import type { MetaFunction } from "@remix-run/node";

export const meta: MetaFunction = ({ data }) => {
  return [
    { title: data?.userProvidedTitle },
    { name: "description", content: data?.userInput },
  ];
};
