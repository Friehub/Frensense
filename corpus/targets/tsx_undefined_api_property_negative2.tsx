// SAFE: API response is validated with a runtime type checker before access

import { z } from 'zod';

const CartItemSchema = z.object({
  name: z.string(),
  priceSnapshot: z.number().positive(),
  quantity: z.number().int().positive()
});

export function CartItem({ item }: { item: any }) {
  const parsed = CartItemSchema.safeParse(item);
  if (!parsed.success) {
    return <div className="cart-item error">Invalid item data</div>;
  }
  const { name, priceSnapshot, quantity } = parsed.data;
  return (
    <div className="cart-item">
      <span className="title">{name}</span>
      <span className="total">Total: {priceSnapshot * quantity}</span>
    </div>
  );
}
