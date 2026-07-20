// SAFE: Omits tabIndex entirely so the button retains native focusability
interface FocusTrapProps {
  onClose: () => void;
}

export function FocusTrap({ onClose }: FocusTrapProps) {
  return (
    <div role="dialog" aria-modal="true">
      <button onClick={onClose}>Close</button>
    </div>
  );
}
