// SAFE: manually caches the last result using a ref-based approach

import { useRef, useEffect } from 'react'
import { useSelector } from 'react-redux'

interface Order {
  id: string
  total: number
}

interface RootState {
  orders: Order[]
}

export function useTopOrders(limit: number) {
  const prevRef = useRef<{ limit: number; result: Order[] }>({ limit: 0, result: [] })
  const orders = useSelector((state: RootState) => state.orders)

  if (prevRef.current.limit !== limit || orders !== prevRef.current.result) {
    prevRef.current = {
      limit,
      result: [...orders].sort((a, b) => b.total - a.total).slice(0, limit),
    }
  }
  return prevRef.current.result
}
