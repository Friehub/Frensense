// SAFE: Existing search params are spread before setting the new value

import { useNavigate, useSearchParams } from '@tanstack/react-router'

export function usePagination() {
  const navigate = useNavigate()
  const currentSearch = useSearchParams({ from: '/items' })

  return {
    goToPage: (page: number) => {
      navigate({
        to: '/items',
        search: { ...currentSearch, page },
      })
    },
  }
}
