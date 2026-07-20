// SAFE: Search params are built incrementally from a base object

import { useNavigate } from '@tanstack/react-router'

interface ItemsSearch {
  page: number
  sort?: string
  filter?: string
}

export function usePagination(baseSearch: ItemsSearch) {
  const navigate = useNavigate()

  return {
    goToPage: (page: number) => {
      navigate({
        to: '/items',
        search: { ...baseSearch, page },
      })
    },
  }
}
