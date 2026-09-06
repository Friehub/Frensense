// [frensense]
// observation: Two Zustand stores are updated sequentially in response to a single event, creating a window where one store is updated but the other is not, leading to inconsistent UI state.
// impact: Users see a partially updated UI — e.g., the cart count increments before the item renders, or an order appears as "paid" before the receipt is generated. React's concurrent rendering can expose this inconsistency.
// improvement: Combine related state into a single store, or use a callback/event to ensure both updates are applied in the same microtask batch.
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

import { useEffect } from 'react';
import { useCartStore } from './cartStore';
import { useNotificationStore } from './notificationStore';

export function AddToCartButton({ productId }: { productId: string }) {
  const addItem = useCartStore((s) => s.addItem);
  const incrementCount = useNotificationStore((s) => s.incrementCount);

  const handleClick = () => {
    addItem(productId);
    incrementCount(1);
  };

  return <button onClick={handleClick}>Add to Cart</button>;
}
