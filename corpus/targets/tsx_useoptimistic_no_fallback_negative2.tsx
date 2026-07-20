// SAFE: uses a try/catch to remove optimistic message and show error toast on failure

'use client'

import { useOptimistic, useRef, useState } from 'react'

type Message = { id: string; text: string; pending?: boolean }

export default function ChatRoom() {
  const formRef = useRef<HTMLFormElement>(null)
  const [error, setError] = useState<string | null>(null)
  const [messages, setMessages] = useOptimistic<Message[], Message>(
    [],
    (state, newMessage) => [...state, newMessage],
  )

  async function sendMessage(formData: FormData) {
    const text = formData.get('text') as string
    const tempId = crypto.randomUUID()
    const optimisticMessage: Message = { id: tempId, text, pending: true }

    setError(null)

    try {
      setMessages(optimisticMessage)
      const res = await fetch('/api/messages', {
        method: 'POST',
        body: JSON.stringify({ text }),
      })
      if (!res.ok) throw new Error(await res.text())
    } catch (e) {
      setError(`Failed: ${(e as Error).message}`)
    }

    formRef.current?.reset()
  }

  return (
    <form ref={formRef} action={sendMessage}>
      <input name="text" required />
      <button type="submit">Send</button>
      {error && <p role="alert">{error}</p>}
      <ul>
        {messages.map((m) => (
          <li key={m.id}>{m.text} {m.pending && '(sending...)'}</li>
        ))}
      </ul>
    </form>
  )
}
