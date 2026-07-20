// [frensense]
// observation: User-controlled URL is accessed via array index before href.
// impact: Array access does not sanitize, allowing javascript: XSS.
// improvement: Validate the array element before assigning to href.
export function UserLink({ url, label }: { url: string[]; label: string }) {
  return <a href={url[0]}>{label}</a>;
}
