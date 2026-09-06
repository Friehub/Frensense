// SAFE: Access-controlled content is gated by server-side condition — the content is never rendered in the DOM for non-premium users

export function PremiumContent({ isPremium }: { isPremium: boolean }) {
  return (
    <div>
      {isPremium && (
        <div>
          <h2>Premium Exclusive Content</h2>
          <p>This content should only be visible to premium users.</p>
        </div>
      )}
    </div>
  );
}
