// [frensense]
// observation: Remix <Link to={userUrl}> renders user-controlled URL directly without validation, enabling open redirect or XSS via javascript: scheme.
// impact: Attacker can inject javascript: URLs for XSS or redirect users to phishing sites by controlling the `to` prop.
// improvement: Validate the URL against an allowlist, reject javascript: and data: schemes, and use encodeURI.

import { Link } from "@remix-run/react";

interface UserLinkProps {
  userUrl: string;
  label: string;
}

export function UserLink({ userUrl, label }: UserLinkProps) {
  return <Link to={userUrl}>{label}</Link>;
}
