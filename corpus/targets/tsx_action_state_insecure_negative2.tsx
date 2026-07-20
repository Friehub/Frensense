// SAFE: uses a mapped return type to ensure only safe fields are exposed

'use server'

interface SafeOrderState {
  orderId: string
  status: string
}

export async function submitOrder(prevState: unknown, data: FormData): Promise<SafeOrderState> {
  const productId = data.get('productId')
  const order = await createOrder(productId)
  return { orderId: order.id, status: 'confirmed' }
}
