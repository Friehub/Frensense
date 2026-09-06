// SAFE: side effects are extracted into a custom hook that wraps the dispatch

import { useCallback, useReducer } from 'react';

interface State { items: string[] }
type Action = { type: 'add'; payload: string };

function reducer(state: State, action: Action): State {
  if (action.type === 'add') {
    return { items: [...state.items, action.payload] };
  }
  return state;
}

function useItemList() {
  const [state, dispatch] = useReducer(reducer, { items: [] });

  const addItem = useCallback(async (item: string) => {
    dispatch({ type: 'add', payload: item });
    await fetch('/api/items', { method: 'POST', body: JSON.stringify([...state.items, item]) });
  }, [state.items]);

  return { items: state.items, addItem };
}

export function ItemList() {
  const { items, addItem } = useItemList();
  return <button onClick={() => addItem('new')}>Add</button>;
}
