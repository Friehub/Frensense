// [frensense]
// observation: User-controlled URL is concatenated before being set as href.
// impact: Concatenation prefix does not sanitize javascript: protocol.
// improvement: Validate after concatenation or use URL constructor.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
import { getUrlFromParams } from './utils';

export function UserLink({ label }: { label: string }) {
  const url = getUrlFromParams();
  return <a href={url + "#nav"}>{label}</a>;
}
