// SAFE: reducer is pure — side effects are handled in useEffect

import { useEffect, useReducer } from 'react';

interface State { items: string[]; saved: boolean }

function saveToServer(items: string[]) {
  return fetch('/api/items', { method: 'POST', body: JSON.stringify(items) });
}

function reducer(state: State, action: { type: 'add'; payload: string }): State {
  return { items: [...state.items, action.payload], saved: false };
}

export function ItemList() {
  const [state, dispatch] = useReducer(reducer, { items: [], saved: false });

  useEffect(() => {
    if (!state.saved) {
      saveToServer(state.items).then(() => dispatch({ type: 'saved' } as any));
    }
  }, [state.items, state.saved]);

  return <button onClick={() => dispatch({ type: 'add', payload: 'new' })}>Add</button>;
}
