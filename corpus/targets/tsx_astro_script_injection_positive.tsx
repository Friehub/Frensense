// [frensense]
// observation: Astro <Script> component renders user-controlled content directly, enabling arbitrary JS execution.
// impact: Attacker injects malicious JavaScript in the page context, leading to full XSS, session theft, and data exfiltration.
// improvement: Never interpolate user content into script tags. Use data attributes and event delegation instead.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

interface UserScriptProps {
  userScriptContent: string;
}

export function UserScript({ userScriptContent }: UserScriptProps) {
  return <script>{userScriptContent}</script>;
}
