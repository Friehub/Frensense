// SAFE: Uses data attributes and client JS instead of interpolating user content into script tags
interface UserBehaviorProps {
  trackingId: string;
}

export function UserBehavior({ trackingId }: UserBehaviorProps) {
  return (
    <div data-tracking-id={trackingId} data-track="enabled">
      Tracked content
    </div>
  );
}
