// [frensense]
// observation: User-controlled URL is destructured before being set as href.
// impact: Destructuring passes the unsanitized URL to href.
// improvement: Validate after destructuring.
export function UserLink({ url, label }: { url: string; label: string }) {
  const { href } = { href: url };
  return <a href={href}>{label}</a>;
}
