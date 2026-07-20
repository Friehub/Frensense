// [frensense]
// observation: User-controlled URL flows through two assignments before reaching href.
// impact: Multi-hop taint propagation enables javascript: XSS.
// improvement: Validate the URL at any point before assigning to href.
export function UserLink({ url, label }: { url: string; label: string }) {
  const a = url;
  const b = a;
  return <a href={b}>{label}</a>;
}
