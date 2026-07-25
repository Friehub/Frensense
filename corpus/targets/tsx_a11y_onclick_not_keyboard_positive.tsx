// [frensense]
// observation: onClick handler added to a non-semantic element without a corresponding onKeyDown handler for keyboard activation.
// impact: Keyboard-only and screen reader users cannot activate the element, causing a functional denial of service.
// improvement: Add onKeyDown handler that triggers the same action on Enter/Space, or use a native <button>.
// cwe: CWE-20
// cvss: 5.3
// owasp: 
// severity: Medium

interface NavItemProps {
  label: string;
  onNavigate: () => void;
}

export function NavItem({ label, onNavigate }: NavItemProps) {
  return (
    <span role="link" onClick={onNavigate}>
      {label}
    </span>
  );
}
