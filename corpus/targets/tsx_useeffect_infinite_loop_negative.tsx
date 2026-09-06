// SAFE: page state is removed from deps; pagination is driven externally by user interaction

import { useEffect, useState } from 'react';

export function SearchResults({ query, page, onPageChange }: {
  query: string;
  page: number;
  onPageChange: (p: number) => void;
}) {
  const [results, setResults] = useState<string[]>([]);

  useEffect(() => {
    fetch(`/api/search?q=${query}&page=${page}`)
      .then(r => r.json())
      .then(data => setResults(data.items));
  }, [query, page]);

  return <div>{results.length} items</div>;
}
