// [frensense]
// observation: An anchor tag's href attribute is set to user-controlled input without validating the protocol.
// impact: An attacker can inject javascript: URLs, leading to XSS when the user clicks the link.
// improvement: Validate that the URL uses an allowed protocol (http, https) before using it in an href.

export function UserLink({ url, label }: { url: string; label: string }) {
  return <a href={url}>{label}</a>;
}
