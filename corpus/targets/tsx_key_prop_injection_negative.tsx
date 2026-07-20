// [frensense]
// observation: array `key` prop is set directly from user input without validation, allowing duplicate keys that cause React to reconcile the wrong components
// impact: wrong component instances are reused, leading to lost state, incorrect DOM updates, or XSS via key-as-attribute injection
// improvement: use a stable, unique identifier for keys instead of raw user input

'use client'

import { useState } from 'react'

interface Item {
  id: string
  text: string
}

let nextId = 0

export default function EditableList() {
  const [items, setItems] = useState<Item[]>([])

  function addItem() {
    const userText = prompt('Enter text:') ?? ''
    // SAFE: key is a programmatically generated unique ID, not user input
    const id = `item-${nextId++}`
    setItems((prev) => [...prev, { id, text: userText }])
  }

  return (
    <div>
      <button onClick={addItem}>Add Item</button>
      <ul>
        {items.map((item) => (
          <li key={item.id}>
            <input defaultValue={item.text} />
          </li>
        ))}
      </ul>
    </div>
  )
}
