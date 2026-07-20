// [frensense]
// observation: A useEffect hook updates a state variable that is listed in its own dependency array, creating an infinite render loop.
// impact: The component re-renders indefinitely, consuming 100% CPU, freezing the UI, and potentially crashing the browser tab.
// improvement: Use the functional updater form of setState, or remove the state variable from the dependency array if the effect only needs to run once.

import { useEffect, useState } from 'react';

export function SearchResults({ query }: { query: string }) {
  const [results, setResults] = useState<string[]>([]);
  const [page, setPage] = useState(1);

  useEffect(() => {
    fetch(`/api/search?q=${query}&page=${page}`)
      .then(r => r.json())
      .then(data => {
        setResults(data.items);
        setPage(p => p + 1);
      });
  }, [query, page]);

  return <div>{results.length} items</div>;
}
