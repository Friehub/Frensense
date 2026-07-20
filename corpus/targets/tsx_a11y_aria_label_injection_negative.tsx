// SAFE: Sanitizes user input before passing to aria-label using DOMPurify
import DOMPurify from "dompurify";

interface AriaLabelProps {
  userDescription: string;
}

export function AriaLabelInput({ userDescription }: AriaLabelProps) {
  const safeLabel = DOMPurify.sanitize(userDescription);
  return (
    <button aria-label={safeLabel}>
      Click me
    </button>
  );
}
