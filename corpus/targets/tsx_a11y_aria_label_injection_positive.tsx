// [frensense]
// observation: aria-label attribute set directly from unsanitized user input, allowing XSS via screen reader injection.
// impact: Assistive technology executes attacker-controlled content, leading to XSS in accessibility APIs.
// improvement: Sanitize dynamic aria-label values using DOMPurify or escape HTML entities before rendering.
// cwe: CWE-20
// cvss: 5.3
// owasp: 
// severity: Medium

interface AriaLabelProps {
  userDescription: string;
}

export function AriaLabelInput({ userDescription }: AriaLabelProps) {
  return (
    <button aria-label={userDescription}>
      Click me
    </button>
  );
}
