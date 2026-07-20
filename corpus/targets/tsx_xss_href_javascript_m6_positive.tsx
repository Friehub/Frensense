// [frensense]
// observation: User-controlled URL is concatenated before being set as href.
// impact: Concatenation prefix does not sanitize javascript: protocol.
// improvement: Validate after concatenation or use URL constructor.
export function UserLink({ url, label }: { url: string; label: string }) {
  return <a href={url + "#nav"}>{label}</a>;
}
