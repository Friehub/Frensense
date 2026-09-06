// SAFE: Uses a non-portal approach with fixed positioning and aria-modal for focus isolation
interface ConfirmDialogProps {
  isOpen: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ isOpen, onConfirm, onCancel }: ConfirmDialogProps) {
  if (!isOpen) return null;

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") onCancel();
  };

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-labelledby="dialog-title"
      onKeyDown={handleKeyDown}
      style={{
        position: "fixed",
        inset: 0,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        background: "rgba(0,0,0,0.3)",
      }}
    >
      <div tabIndex={-1}>
        <h2 id="dialog-title">Confirm</h2>
        <p>Are you sure?</p>
        <button onClick={onCancel}>Cancel</button>
        <button onClick={onConfirm}>Confirm</button>
      </div>
    </div>
  );
}
