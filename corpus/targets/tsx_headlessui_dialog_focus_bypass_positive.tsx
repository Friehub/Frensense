// [frensense]
// observation: Headless UI Dialog uses a portal but fails to trap focus within the dialog, allowing interaction with background elements.
// impact: Clickjacking — attackers can overlay invisible dialog elements over legitimate UI, tricking users into clicking actions they did not intend.
// improvement: Use the Dialog's built-in focus trapping via initialFocus and properly manage portal rendering with aria-modal.

import { Dialog } from "@headlessui/react";

interface ConfirmDialogProps {
  isOpen: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ isOpen, onConfirm, onCancel }: ConfirmDialogProps) {
  return (
    <Dialog open={isOpen} onClose={onCancel}>
      <div>
        <Dialog.Title>Confirm</Dialog.Title>
        <Dialog.Description>Are you sure?</Dialog.Description>
        <button onClick={onCancel}>Cancel</button>
        <button onClick={onConfirm}>Confirm</button>
      </div>
    </Dialog>
  );
}
