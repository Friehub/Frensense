// [frensense]
// observation: `useOptimistic` applies the optimistic value immediately but has no error rollback when the server action fails
// impact: users see incorrect data permanently when the server rejects the update, leading to confusion and stale UI state
// improvement: wrap optimistic update in a try/catch and roll back state on error

'use client'

import { useOptimistic, useRef } from 'react'

type Message = { id: string; text: string; pending?: boolean }

export default function ChatRoom() {
  const formRef = useRef<HTMLFormElement>(null)
  const [messages, setMessages] = useOptimistic<Message[], Message>(
    [],
    (state, newMessage) => [...state, newMessage],
  )

  async function sendMessage(formData: FormData) {
    const text = formData.get('text') as string
    const tempId = crypto.randomUUID()
    const optimisticMessage: Message = { id: tempId, text, pending: true }

    setMessages(optimisticMessage)

    const res = await fetch('/api/messages', {
      method: 'POST',
      body: JSON.stringify({ text }),
    })

    if (!res.ok) {
      // No rollback — optimistic message stays in UI permanently
      console.error('Failed to send message')
    }

    formRef.current?.reset()
  }

  return (
    <form ref={formRef} action={sendMessage}>
      <input name="text" required />
      <button type="submit">Send</button>
      <ul>
        {messages.map((m) => (
          <li key={m.id}>{m.text} {m.pending && '(sending...)'}</li>
        ))}
      </ul>
    </form>
  )
}
