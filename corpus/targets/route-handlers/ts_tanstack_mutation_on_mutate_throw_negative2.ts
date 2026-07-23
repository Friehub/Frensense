// SAFE: No optimistic update in onMutate, so no risk of throwing

import { useMutation } from '@tanstack/react-query'

interface Todo {
  id: string
  text: string
  done: boolean
}

export function useToggleTodo() {
  return useMutation({
    mutationFn: async (todo: Todo) => {
      const res = await fetch(`/api/todos/${todo.id}`, {
        method: 'PATCH',
        body: JSON.stringify({ done: !todo.done }),
      })
      return res.json()
    },
  })
}
