// SAFE: Uses a unique key per toast to ensure each has its own closure scope, and the action handler captures the correct request id

import * as Toast from '@radix-ui/react-toast';
import { Button } from '@/components/ui/button';
import * as React from 'react';

interface FriendRequest {
  id: string;
  name: string;
}

export function FriendRequestToasts({ requests }: { requests: FriendRequest[] }) {
  const [activeRequests, setActiveRequests] = React.useState<FriendRequest[]>([]);

  React.useEffect(() => {
    if (requests.length > 0) setActiveRequests((prev) => [...prev, ...requests]);
  }, [requests]);

  const dismiss = (id: string) => {
    setActiveRequests((prev) => prev.filter((r) => r.id !== id));
  };

  return (
    <Toast.Provider>
      {activeRequests.map((req) => (
        <Toast.Root key={req.id} open onOpenChange={() => dismiss(req.id)}>
          <Toast.Title>Friend Request</Toast.Title>
          <Toast.Description>{req.name} wants to be your friend.</Toast.Description>
          <Toast.Action asChild altText="Approve">
            <Button onClick={() => fetch(`/api/friends/approve/${req.id}`, { method: 'POST' })}>
              Approve
            </Button>
          </Toast.Action>
        </Toast.Root>
      ))}
      <Toast.Viewport />
    </Toast.Provider>
  );
}
