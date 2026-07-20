// SAFE: uses functional updater to derive new state without adding it as a dependency

import { useEffect, useState, useCallback } from 'react';

export function SearchResults({ query }: { query: string }) {
  const [results, setResults] = useState<string[]>([]);
  const [page, setPage] = useState(1);

  const fetchPage = useCallback((pageNum: number) => {
    fetch(`/api/search?q=${query}&page=${pageNum}`)
      .then(r => r.json())
      .then(data => setResults(data.items));
  }, [query]);

  useEffect(() => {
    fetchPage(page);
  }, [page, fetchPage]);

  return <div>{results.length} items</div>;
}
