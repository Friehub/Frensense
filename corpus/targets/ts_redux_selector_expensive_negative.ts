// SAFE: memoizes the derived computation with createSelector

import { useSelector } from 'react-redux'
import { createSelector } from '@reduxjs/toolkit'

interface Order {
  id: string
  total: number
}

interface RootState {
  orders: Order[]
}

const selectOrders = (state: RootState) => state.orders

const selectTopOrders = createSelector(
  [selectOrders, (_state: RootState, limit: number) => limit],
  (orders, limit) => [...orders].sort((a, b) => b.total - a.total).slice(0, limit),
)

export function useTopOrders(limit: number) {
  return useSelector((state: RootState) => selectTopOrders(state, limit))
}
