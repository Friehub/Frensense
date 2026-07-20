// SAFE: Uses a boolean flag to prevent setState after unmount, combined with a custom useFetch hook for reuse.

import { useEffect, useState } from 'react';

function useFetch<T>(url: string): { data: T | null; loading: boolean; error: Error | null } {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    let cancelled = false;
    const abortController = new AbortController();

    setLoading(true);
    fetch(url, { signal: abortController.signal })
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((json) => {
        if (!cancelled) setData(json);
      })
      .catch((err) => {
        if (err instanceof DOMException && err.name === 'AbortError') return;
        if (!cancelled) setError(err);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
      abortController.abort();
    };
  }, [url]);

  return { data, loading, error };
}

interface Product {
  id: string;
  name: string;
  price: number;
}

export function ProductList({ category }: { category: string }) {
  const { data: products, loading, error } = useFetch<Product[]>(
    `/api/products?category=${encodeURIComponent(category)}`
  );

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <ul>
      {products?.map((p) => (
        <li key={p.id}>{p.name} — ${p.price}</li>
      ))}
    </ul>
  );
}
