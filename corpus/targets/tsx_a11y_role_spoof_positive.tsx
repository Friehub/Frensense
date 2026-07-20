// [frensense]
// observation: A non-interactive <div> is given role="button" without keyboard event handlers, spoofing interactive semantics.
// impact: Screen reader users are misled into believing the element is interactive, enabling clickjacking or phishing.
// improvement: Use a native <button> element or add proper keyboard handling (onKeyDown, tabIndex, role).

interface ClickableCardProps {
  onClick: () => void;
  children: React.ReactNode;
}

export function ClickableCard({ onClick, children }: ClickableCardProps) {
  return (
    <div role="button" onClick={onClick}>
      {children}
    </div>
  );
}
