// SAFE: Both related pieces of state are in a single store, so updates are always atomic and consistent

import { create } from 'zustand';
import { useEffect } from 'react';

interface CartWithNotificationStore {
  items: string[];
  notificationCount: number;
  addItem: (productId: string) => void;
}

const useCartWithNotificationStore = create<CartWithNotificationStore>()((set) => ({
  items: [],
  notificationCount: 0,
  addItem: (productId) =>
    set((state) => ({
      items: [...state.items, productId],
      notificationCount: state.notificationCount + 1,
    })),
}));

export function AddToCartButton({ productId }: { productId: string }) {
  const addItem = useCartWithNotificationStore((s) => s.addItem);

  const handleClick = () => {
    addItem(productId);
  };

  return <button onClick={handleClick}>Add to Cart</button>;
}
