// [frensense]
// observation: tabIndex={-1} applied to a focusable element, intentionally removing it from sequential keyboard navigation.
// impact: Keyboard-only users cannot reach the element, creating a keyboard trap and violating WCAG 2.1.1.
// improvement: Use tabIndex={0} for natural focus order or remove tabIndex entirely for native focusability.

interface FocusTrapProps {
  onClose: () => void;
}

export function FocusTrap({ onClose }: FocusTrapProps) {
  return (
    <div role="dialog" aria-modal="true">
      <button tabIndex={-1} onClick={onClose}>Close</button>
    </div>
  );
}
