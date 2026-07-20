// [frensense]
// observation: User-controlled URL is injected into href via template literal.
// impact: Template literal passes javascript: URL to href unsanitized.
// improvement: Validate the URL before assigning to href.
export function UserLink({ url, label }: { url: string; label: string }) {
  return <a href={`${url}`}>{label}</a>;
}
