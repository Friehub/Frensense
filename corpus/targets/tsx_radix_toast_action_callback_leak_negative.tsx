// SAFE: Each request ID is bound to its specific toast via a callback that captures the correct id at creation time

import * as Toast from '@radix-ui/react-toast';
import { Button } from '@/components/ui/button';
import * as React from 'react';

interface FriendRequest {
  id: string;
  name: string;
}

function RequestToast({ request, onClose }: { request: FriendRequest; onClose: () => void }) {
  return (
    <Toast.Root open onOpenChange={onClose}>
      <Toast.Title>Friend Request</Toast.Title>
      <Toast.Description>{request.name} wants to be your friend.</Toast.Description>
      <Toast.Action asChild altText="Approve friend request">
        <Button onClick={() => fetch(`/api/friends/approve/${request.id}`, { method: 'POST' })}>
          Approve
        </Button>
      </Toast.Action>
    </Toast.Root>
  );
}

export function FriendRequestToasts({ requests }: { requests: FriendRequest[] }) {
  const [currentIndex, setCurrentIndex] = React.useState(0);

  if (currentIndex >= requests.length) return null;

  return (
    <Toast.Provider>
      <RequestToast
        request={requests[currentIndex]}
        onClose={() => setCurrentIndex((i) => i + 1)}
      />
      <Toast.Viewport />
    </Toast.Provider>
  );
}
