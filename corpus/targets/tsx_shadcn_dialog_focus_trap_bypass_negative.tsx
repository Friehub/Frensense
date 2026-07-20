// SAFE: Uses shadcn/ui Dialog which internally wraps @radix-ui/react-dialog with proper focus trapping enabled

import { Dialog, DialogContent, DialogTrigger, DialogClose } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';

export function ConfirmDeleteDialog({ itemId, onDelete }: { itemId: string; onDelete: (id: string) => void }) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="destructive">Delete Item</Button>
      </DialogTrigger>
      <DialogContent>
        <h2>Confirm Deletion</h2>
        <p>Are you sure you want to delete this item? This action cannot be undone.</p>
        <div style={{ display: 'flex', gap: '8px' }}>
          <Button onClick={() => onDelete(itemId)}>Delete</Button>
          <DialogClose asChild>
            <Button>Cancel</Button>
          </DialogClose>
        </div>
      </DialogContent>
    </Dialog>
  );
}
