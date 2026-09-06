// SAFE: optimistic update is rolled back on server error using a reducer fallback

'use client'

import { useOptimistic, useRef, useReducer, startTransition } from 'react'

type Message = { id: string; text: string; pending?: boolean }

function messagesReducer(state: Message[], action: { type: 'add' | 'remove'; message: Message }) {
  switch (action.type) {
    case 'add':
      return [...state, action.message]
    case 'remove':
      return state.filter((m) => m.id !== action.message.id)
    default:
      return state
  }
}

export default function ChatRoom() {
  const formRef = useRef<HTMLFormElement>(null)
  const [actualMessages, dispatch] = useReducer(messagesReducer, [])
  const [optimisticMessages, setOptimistic] = useOptimistic(
    actualMessages,
    (state, newMessage: Message) => [...state, newMessage],
  )

  async function sendMessage(formData: FormData) {
    const text = formData.get('text') as string
    const tempId = crypto.randomUUID()
    const optimisticMessage: Message = { id: tempId, text, pending: true }

    startTransition(() => setOptimistic(optimisticMessage))

    try {
      const res = await fetch('/api/messages', {
        method: 'POST',
        body: JSON.stringify({ text }),
      })
      if (!res.ok) throw new Error('Server rejected')
      const saved = await res.json()
      dispatch({ type: 'add', message: { ...saved, pending: false } })
    } catch {
      dispatch({ type: 'remove', message: optimisticMessage })
    }

    formRef.current?.reset()
  }

  return (
    <form ref={formRef} action={sendMessage}>
      <input name="text" required />
      <button type="submit">Send</button>
      <ul>
        {optimisticMessages.map((m) => (
          <li key={m.id}>{m.text} {m.pending && '(sending...)'}</li>
        ))}
      </ul>
    </form>
  )
}
