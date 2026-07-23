// [frensense]
// observation: an optimistic update does not await the server response to reconcile the final state, so a slow server response can set data back to a stale value after a newer update
// impact: the UI shows wrong final state — a user's second edit is overwritten by the server response to their first edit due to race conditions
// improvement: use the server response data in `onSettled` to reconcile, or use `onMutate` snapshot with rollback on error

import { useMutation, useQueryClient } from '@tanstack/react-query'

interface Todo {
  id: string
  text: string
  done: boolean
}

export function useToggleTodo() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (todo: Todo) => {
      const res = await fetch(`/api/todos/${todo.id}`, {
        method: 'PATCH',
        body: JSON.stringify({ done: !todo.done }),
      })
      return res.json()
    },
    onMutate: async (todo: Todo) => {
      queryClient.setQueryData<Todo[]>(['todos'], (old) =>
        old?.map((t) => (t.id === todo.id ? { ...t, done: !t.done } : t)),
      )
    },
  })
}
