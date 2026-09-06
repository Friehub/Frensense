// [frensense]
// observation: A shadcn/ui Dialog is implemented without a proper focus trap — pressing Tab cycles focus outside the dialog onto background elements, allowing keyboard navigation to escape the modal.
// impact: Users can tab past the dialog and interact with background UI elements while the modal is still open, violating modal isolation. Keyboard-only users can accidentally trigger actions behind the dialog, including clicks on hidden submit buttons or navigation links.
// improvement: Use the shadcn `Dialog` component from `@radix-ui/react-dialog` which includes built-in focus trapping, or add a manual focus trap via `onKeyDown` handling with `FocusTrap` from `focus-trap-react`.

import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
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
          <Button>Cancel</Button>
        </div>
        <div tabIndex={0} style={{ opacity: 0, position: 'absolute' }}>
          Focus escape element
        </div>
      </DialogContent>
    </Dialog>
  );
}
