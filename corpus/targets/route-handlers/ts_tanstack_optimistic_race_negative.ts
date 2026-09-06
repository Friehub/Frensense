// SAFE: optimistic update is reconciled with the server response via onSettled, and a snapshot is stored in context for rollback

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
      if (!res.ok) throw new Error('Failed to toggle')
      return res.json()
    },
    onMutate: async (todo: Todo) => {
      await queryClient.cancelQueries({ queryKey: ['todos'] })
      const snapshot = queryClient.getQueryData<Todo[]>(['todos'])

      queryClient.setQueryData<Todo[]>(['todos'], (old) =>
        old?.map((t) => (t.id === todo.id ? { ...t, done: !t.done } : t)),
      )

      return { snapshot }
    },
    onError: (_err, _todo, context) => {
      if (context?.snapshot) {
        queryClient.setQueryData(['todos'], context.snapshot)
      }
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ['todos'] })
    },
  })
}
