// SAFE: A manual focus trap is implemented using onKeyDown to intercept Tab and keep focus within the dialog

import { useCallback, useRef, useEffect } from 'react';
import { Button } from '@/components/ui/button';

interface FocusTrapDialogProps {
  open: boolean;
  onClose: () => void;
  itemId: string;
  onDelete: (id: string) => void;
}

export function FocusTrapConfirmDialog({ open, onClose, itemId, onDelete }: FocusTrapDialogProps) {
  const dialogRef = useRef<HTMLDivElement>(null);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
        return;
      }
      if (e.key === 'Tab') {
        const focusable = dialogRef.current?.querySelectorAll<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        if (!focusable || focusable.length === 0) return;
        const first = focusable[0];
        const last = focusable[focusable.length - 1];
        if (e.shiftKey && document.activeElement === first) {
          e.preventDefault();
          last.focus();
        } else if (!e.shiftKey && document.activeElement === last) {
          e.preventDefault();
          first.focus();
        }
      }
    },
    [onClose]
  );

  useEffect(() => {
    if (open) setTimeout(() => dialogRef.current?.querySelector<HTMLElement>('button')?.focus(), 0);
  }, [open]);

  if (!open) return null;

  return (
    <div role="dialog" aria-modal="true" ref={dialogRef} onKeyDown={handleKeyDown}
      style={{ position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.5)', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <div style={{ background: 'white', padding: '24px', borderRadius: '8px', minWidth: '320px' }}>
        <h2>Confirm Deletion</h2>
        <p>Are you sure you want to delete this item?</p>
        <div style={{ display: 'flex', gap: '8px', marginTop: '16px' }}>
          <Button onClick={() => { onDelete(itemId); onClose(); }}>Delete</Button>
          <Button onClick={onClose}>Cancel</Button>
        </div>
      </div>
    </div>
  );
}
