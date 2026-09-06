// SAFE: Uses tabIndex={0} to keep close button in natural focus order
interface FocusTrapProps {
  onClose: () => void;
}

export function FocusTrap({ onClose }: FocusTrapProps) {
  return (
    <div role="dialog" aria-modal="true">
      <button tabIndex={0} onClick={onClose}>Close</button>
    </div>
  );
}
