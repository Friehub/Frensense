// [frensense]
// observation: A Toast's `action` onClick handler captures a closure variable that becomes stale by the time the user clicks the action button, causing the wrong action to execute (e.g., approving the wrong friend request, confirming the wrong deletion).
// impact: In a multi-toast scenario (e.g., multiple pending friend requests), all toast action buttons capture the last value of the closure variable. When the user clicks any action, the same (last) action fires, potentially approving the wrong user's request or performing the wrong operation.
// improvement: Ensure each Toast action captures its own unique scope, either by using a separate component or by binding the correct value at creation time.

import * as Toast from '@radix-ui/react-toast';
import { Button } from '@/components/ui/button';
import * as React from 'react';

interface FriendRequest {
  id: string;
  name: string;
}

export function FriendRequestToasts({ requests }: { requests: FriendRequest[] }) {
  const [open, setOpen] = React.useState(false);
  const [currentRequest, setCurrentRequest] = React.useState<FriendRequest | null>(null);
  let pendingRequestId: string;

  const showNext = () => {
    const next = requests.pop();
    if (!next) return;
    pendingRequestId = next.id;
    setCurrentRequest(next);
    setOpen(true);
  };

  const handleApprove = () => {
    fetch(`/api/friends/approve/${pendingRequestId}`, { method: 'POST' });
    setOpen(false);
  };

  return (
    <Toast.Provider>
      <Button onClick={showNext}>Show Next Request</Button>
      <Toast.Root open={open} onOpenChange={setOpen}>
        <Toast.Title>Friend Request</Toast.Title>
        <Toast.Description>{currentRequest?.name} wants to be your friend.</Toast.Description>
        <Toast.Action asChild altText="Approve friend request">
          <Button onClick={handleApprove}>Approve</Button>
        </Toast.Action>
      </Toast.Root>
      <Toast.Viewport />
    </Toast.Provider>
  );
}
