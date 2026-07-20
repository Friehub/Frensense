// SAFE: server action returns only safe fields to the client

'use server'

export async function submitOrder(prevState: unknown, data: FormData) {
  const productId = data.get('productId')
  const order = await createOrder(productId)
  return { orderId: order.id, status: 'confirmed' }
}
