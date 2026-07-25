// [frensense]
// observation: User-controlled input is passed to the `composes` property in CSS Modules. The `composes` directive can reference other CSS module files via relative paths, allowing an attacker to perform path traversal (e.g., `composes: ... from './../../secret/styles.module.css'`) and include arbitrary CSS content or leak file structure information.
// impact: An attacker can read arbitrary CSS files from the server by exploiting path traversal through the `composes ... from` directive. This can leak internal stylesheet conventions, embedded API URLs, or commented-out sensitive data in CSS files. In some CSS module configurations, this can also be used to perform local file inclusion.
// improvement: Validate and sanitize any user input used in CSS module `composes` paths. Restrict composition paths to a predefined allowlist of approved CSS module references.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client';

import styles from './ThemedWidget.module.css';

export function ThemedWidget({ composeFrom }: { composeFrom: string }) {
  return (
    <div className={styles.widget}>
      <style>{`.composed { composes: base from "${composeFrom}"; }`}</style>
      <div className="composed">Widget Content</div>
    </div>
  );
}
