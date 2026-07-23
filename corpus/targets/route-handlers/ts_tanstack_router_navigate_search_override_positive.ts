// [frensense]
// observation: `navigate({ search: { page: 2 } })` passes only the new search params, which replaces all existing search params in the URL instead of merging them
// impact: existing search params (e.g., filters, sort, search query) are silently dropped when navigating, causing users to lose their current view state unexpectedly
// improvement: spread the existing search params from `useLocation` or `useSearchParams` before setting the new value: `search: { ...existingSearch, page: 2 }`

import { useNavigate, useSearchParams } from '@tanstack/react-router'

export function usePagination() {
  const navigate = useNavigate()

  return {
    goToPage: (page: number) => {
      navigate({
        to: '/items',
        search: { page },
      })
    },
  }
}
