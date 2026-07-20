// SAFE: Escapes HTML entities in aria-label to prevent XSS injection
function escapeHtml(str: string): string {
  return str.replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#x27;");
}

interface AriaLabelProps {
  userDescription: string;
}

export function AriaLabelInput({ userDescription }: AriaLabelProps) {
  const safeLabel = escapeHtml(userDescription);
  return (
    <button aria-label={safeLabel}>
      Click me
    </button>
  );
}
