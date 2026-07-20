// [frensense]
// observation: User-controlled URL passes through a helper function that does not validate protocols before href.
// impact: Helper returns dangerous javascript: URL directly to href.
// improvement: Add protocol validation in the helper function.
function processUrl(x: string): string { return x; }
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = processUrl(url);
  return <a href={u}>{label}</a>;
}
