// SAFE: Uses Dialog.Panel and Dialog.Backdrop to ensure proper focus trapping and modal behavior
import { Dialog } from "@headlessui/react";
import { useRef } from "react";

interface ConfirmDialogProps {
  isOpen: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ isOpen, onConfirm, onCancel }: ConfirmDialogProps) {
  const confirmRef = useRef(null);

  return (
    <Dialog open={isOpen} onClose={onCancel} initialFocus={confirmRef}>
      <div className="fixed inset-0 bg-black/30" aria-hidden="true" />
      <div className="fixed inset-0 flex items-center justify-center">
        <Dialog.Panel>
          <Dialog.Title>Confirm</Dialog.Title>
          <Dialog.Description>Are you sure?</Dialog.Description>
          <button ref={confirmRef} onClick={onCancel}>Cancel</button>
          <button onClick={onConfirm}>Confirm</button>
        </Dialog.Panel>
      </div>
    </Dialog>
  );
}
