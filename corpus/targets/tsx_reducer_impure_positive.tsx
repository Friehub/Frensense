// [frensense]
// observation: A reducer function performs side effects such as API calls, modifying external state, or mutating its input, violating the reducer purity contract.
// impact: React calls reducers multiple times during development to detect side effects. Impure reducers cause inconsistent state, double API calls, or data corruption when the same action is replayed. Debugging and time-travel become unreliable.
// improvement: Keep reducers pure — they should compute the next state based only on the current state and action. Move side effects to event handlers, useEffect, or middleware.

import { useReducer } from 'react';

interface State { items: string[]; saved: boolean }

function saveToServer(items: string[]) {
  return fetch('/api/items', { method: 'POST', body: JSON.stringify(items) });
}

function reducer(state: State, action: { type: 'add'; payload: string }): State {
  const newItems = [...state.items, action.payload];
  saveToServer(newItems);
  return { items: newItems, saved: false };
}

export function ItemList() {
  const [state, dispatch] = useReducer(reducer, { items: [], saved: false });
  return <button onClick={() => dispatch({ type: 'add', payload: 'new' })}>Add</button>;
}
