// [frensense]
// observation: `onMutate` performs an async operation (e.g., optimistic cache update) and throws an error without a try-catch, so the rejection propagates as an unhandled promise rejection
// impact: the mutation's `onError` handler is never called, the optimistic update is never rolled back, and the UI remains in an incorrect state after the failure — plus an unhandled promise rejection warning in the console
// improvement: wrap `onMutate` logic in a try-catch block and return the rollback context, or avoid throwing in `onMutate`
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

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
      const previousTodos = queryClient.getQueryData<Todo[]>(['todos'])
      queryClient.setQueryData<Todo[]>(['todos'], (old) =>
        old?.map((t) => (t.id === todo.id ? { ...t, done: !t.done } : t)),
      )
      throw new Error('failed to cache')
    },
  })
}
