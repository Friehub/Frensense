// SAFE: Uses React.startTransition to batch the two store updates together, preventing intermediate inconsistent states

import { startTransition, useCallback } from 'react';
import { useCartStore } from './cartStore';
import { useNotificationStore } from './notificationStore';

export function AddToCartButton({ productId }: { productId: string }) {
  const addItem = useCartStore((s) => s.addItem);
  const incrementCount = useNotificationStore((s) => s.incrementCount);

  const handleClick = useCallback(() => {
    startTransition(() => {
      addItem(productId);
      incrementCount(1);
    });
  }, [productId, addItem, incrementCount]);

  return <button onClick={handleClick}>Add to Cart</button>;
}
