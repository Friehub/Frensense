// [frensense]
// observation: selector performs an expensive array sort/filter on every store subscription
// impact: UI jank; unnecessary O(n log n) computation on every state change
// improvement: memoize with createSelector or reselect

import { useSelector } from 'react-redux'

interface Order {
  id: string
  total: number
}

interface RootState {
  orders: Order[]
}

export function useTopOrders(limit: number) {
  return useSelector((state: RootState) =>
    [...state.orders].sort((a, b) => b.total - a.total).slice(0, limit),
  )
}
